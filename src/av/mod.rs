use libc::{c_int, c_uint, c_void};

use std::sync::Arc;
use std::cell::{RefCell, UnsafeCell};
use std::error::Error;
use std::{fmt, raw, slice, mem};
use std::old_io::timer as timer;
use std::time::Duration;
use core::{Tox};

pub use self::AvEvent::*;

mod ll;

#[repr(C)]
#[derive(Copy, Debug)]
pub enum CallType {
    Audio = 192,
    Video
}

#[repr(C)]
#[derive(Copy, Debug)]
pub enum CallState {
    Inviting,
    Starting,
    Active,
    Hold,
    HungUp,
}

#[repr(C)]
#[derive(Copy, Debug)]
pub enum AvError {
    Unknown = -1,
    NoCall = -20,
    InvalidState = -21,
    AlreadyInCallWithPeer = -22,
    ReachedCallLimit = -23,
    ErrorInitializingCodecs = -30,
    ErrorSettingVideoResolution = -31,
    ErrorSettingVideoBitrate = -32,
    ErrorSplittingVideoPayload = -33,
    ErrorEncodingVideo = -34,
    ErrorEncodingAudio = -35,
    ErrorSendingPayload = -40,
    ErrorCreatingRtpSessions = -41,
    NoRtpSession = -50,
    InvalidCodecState = -51,
    PacketTooLarge = -52,
}

impl Error for AvError {
    fn description(&self) -> &str {
        match *self {
            AvError::Unknown => "unknown error",
            AvError::NoCall => "no such call",
            AvError::InvalidState => "invalid call state",
            AvError::AlreadyInCallWithPeer => "already in call with this peer",
            AvError::ReachedCallLimit => "reached call limit",
            AvError::ErrorInitializingCodecs => "failed to initalize codecs",
            AvError::ErrorSettingVideoResolution => "faied to set video resolution",
            AvError::ErrorSettingVideoBitrate => "failed to set video bitrate",
            AvError::ErrorSplittingVideoPayload => "failed to split video payload",
            AvError::ErrorEncodingVideo => "failed to encode video",
            AvError::ErrorEncodingAudio => "failed to encode audio",
            AvError::ErrorSendingPayload => "failed to send payload",
            AvError::ErrorCreatingRtpSessions => "failed to create a rtp session",
            AvError::NoRtpSession => "no rtp session",
            AvError::InvalidCodecState => "invalid codec state",
            AvError::PacketTooLarge => "the packet is too large",
        }
    }
}

impl fmt::Display for AvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
pub enum Capability {
    AudioEncoding = 1 << 0,
    AudioDecoding = 1 << 1,
    VideoEncoding = 1 << 2,
    VideoDecoding = 1 << 3,
}

#[repr(C)]
#[derive(Copy)]
pub struct CallSettings {
    pub call_type: CallType,
    pub video_bitrate: u32,
    pub max_video_width: u16,
    pub max_video_height: u16,
    pub audio_bitrate: u32,
    pub audio_frame_duration: u16,
    pub audio_sample_rate: u32,
    pub audio_channels: u32,
}

/// ToxAv events from a given friend (call id included).
#[derive(Clone, Copy, Debug)]
pub enum AvEvent {
    Invite(i32),
    Ringing(i32),
    Start(i32),
    Cancel(i32),
    Reject(i32),
    End(i32),
    RequestTimeout(i32),
    PeerTimeout(i32),
    PeerCsChange(i32),
    SelfCsChange(i32),
}

/// Audio, received in group audio callback
pub struct AudioBit<'a> {
    pub pcm: &'a [i16],
    pub samples: u32,
    pub channels: u8,
    pub sample_rate: u32,
}

pub type FnEvent = FnMut(&ToxAv, AvEvent) + 'static;
pub type FnAudio = FnMut(&ToxAv, i32, &[i16]) + 'static;
pub type FnGroupAudio = FnMut(&mut Tox, i32, i32, AudioBit) + 'static;


pub struct GroupAudio {
    closure: *mut Box<FnGroupAudio>,
    poison: Arc<UnsafeCell<bool>>, // mb use AtomicBool instead?
}

impl GroupAudio {
    pub fn new_groupchat(&self, tox: &mut Tox) -> Option<i32> {
        unsafe {
            assert!(*self.poison.get() == false, "tried to use a poisoned GroupAudio");
            let closure: *mut c_void = mem::transmute(self.closure);
            match ll::toxav_add_av_groupchat(tox.raw(), Some(group_audio_callback), closure) {
                -1 => None,
                n => Some(n),
            }
        }
    }
    pub fn join_groupchat(&self, tox: &mut Tox, fr_num: i32, data: &[u8]) -> Option<i32> {
        unsafe {
            assert!(*self.poison.get() == false, "tried to use a poisoned GroupAudio");
            let closure: *mut c_void = mem::transmute(self.closure);
            let res = ll::toxav_join_av_groupchat(
                tox.raw(), fr_num, data.as_ptr(), data.len() as u16,
                Some(group_audio_callback), closure
            );
            match res {
                -1 => None,
                n => Some(n),
            }
        }
    }
    pub fn send_audio(&self, tox: &mut Tox, gr_num: i32, bit: AudioBit) -> Result<(),()> {
        unsafe {
            assert!(*self.poison.get() == false, "tried to use a poisoned GroupAudio");
            let res = ll::toxav_group_send_audio(
                tox.raw(), gr_num, bit.pcm.as_ptr(), bit.samples, bit.channels, bit.sample_rate
            );
            match res {
                -1 => Err(()),
                _ => Ok(()),
            }
        }
    }
}

extern fn group_audio_callback(tox_ll: *mut ll::Tox, gr_num: c_int, peer_num: c_int, pcm: *const i16,
                               samples: c_uint, channels: u8, rate: c_uint, data: *mut c_void) {
    unsafe {
        let mut closure: &mut FnGroupAudio =
            mem::transmute_copy(mem::transmute::<_, &mut raw::Closure>(data));
        let bit = AudioBit {
            pcm: slice::from_raw_parts(pcm, (samples * channels as u32) as usize),
            samples: samples as u32,
            channels: channels,
            sample_rate: rate,
        };
        let mut tox: Tox = Tox::from_raw_tox(tox_ll);
        (*closure)(&mut tox, gr_num as i32, peer_num as i32, bit);
        mem::forget(tox);
    }
}


pub struct Transmittion<'a> {
    av: &'a ToxAv,
    call_id: i32,
}

macro_rules! av_result {
    ($exp:expr, $res:expr) => {
        match $exp {
            0 => Ok($res),
            e => Err(::std::mem::transmute::<_, AvError>(e))
        }
    };
    ($exp:expr) => { av_result!($exp, ()) };
}

#[unsafe_destructor]
impl<'a> Drop for Transmittion<'a> {
    fn drop(&mut self) {
        unsafe {
            ll::toxav_stop_call(self.av.av, self.call_id);
        }
    }
}

impl<'a> Transmittion<'a> {
    pub fn encode_audio_frame(&self, dest: &mut [u8], frame: &[i16]) -> Result<usize, AvError> {
        unsafe {
            let res = ll::toxav_prepare_audio_frame(
                self.av.av, self.call_id,
                dest.as_mut_ptr(), dest.len() as i32,
                frame.as_ptr(), frame.len() as i32
            );
            match res {
                err if err < 0 => Err(mem::transmute(err)),
                size => Ok(size as usize),
            }
        }
    }
    pub fn send_audio(&self, frame: &[u8]) -> Result<(), AvError> {
        unsafe {
            let res = ll::toxav_send_audio(
                self.av.av, self.call_id,
                frame.as_ptr(), frame.len() as c_uint
            );
            av_result!(res)
        }
    }
    pub fn kill(self) {
        unsafe {
            ll::toxav_kill_transmission(self.av.av, self.call_id);
        }
    }
}


pub struct ToxAv {
    av: *mut ll::ToxAv,
    tox: Arc<RefCell<Tox>>,
    on_event: Option<Box<Box<FnEvent>>>,
    on_audio: Option<Box<Box<FnAudio>>>,
    on_group_audio: Option<Box<Box<FnGroupAudio>>>,
    poison: Arc<UnsafeCell<bool>>,
}

unsafe impl Send for ToxAv {}
unsafe impl Sync for ToxAv {}

impl Drop for ToxAv {
    fn drop(&mut self) {
        use super::core::GroupchatType;
        unsafe {
            // Try to remove all av groups before drop to avoid crash
            // Let's pray it will somehow help
            *self.poison.get() = true;
            let tox = self.tox.as_unsafe_cell().get().as_mut().unwrap();
            let lst = tox.get_chatlist().into_iter();
            for n in lst {
                if tox.group_get_type(n) == Some(GroupchatType::Av) {
                    drop(tox.del_groupchat(n));
                }
            }

            ll::toxav_kill(self.av);
        }
    }
}

impl ToxAv {
    pub fn new(mut tox: Tox, max_calls: i32) -> (Arc<RefCell<Tox>>, ToxAv) {
        let av = ToxAv {
            av: unsafe { ll::toxav_new(tox.raw(), max_calls) },
            tox: Arc::new(RefCell::new(tox)),
            on_event: None,
            on_audio: None,
            on_group_audio: None,
            poison: Arc::new(UnsafeCell::new(false)),
        };

        (av.tox.clone(), av)
    }

    pub fn on_event(&mut self, on_event: Box<FnEvent>) {
        use self::ll::ToxAvCallbackID::*;
        use self::ll::toxav_register_callstate_callback as reg_cb;
        unsafe {
            let closure = box on_event;
            let data: *mut c_void = mem::transmute(&*closure);

            reg_cb(self.av, Some(on_invite), OnInvite, data);
            reg_cb(self.av, Some(on_ringing), OnRinging, data);
            reg_cb(self.av, Some(on_start), OnStart, data);
            reg_cb(self.av, Some(on_cancel), OnCancel, data);
            reg_cb(self.av, Some(on_reject), OnReject, data);
            reg_cb(self.av, Some(on_end), OnEnd, data);
            reg_cb(self.av, Some(on_request_timeout), OnRequestTimeout, data);
            reg_cb(self.av, Some(on_peer_timeout), OnPeerTimeout, data);
            reg_cb(self.av, Some(on_peer_cs_change), OnPeerCSChange, data);
            reg_cb(self.av, Some(on_self_cs_change), OnSelfCSChange, data);

            self.on_event = Some(closure);
        }
    }

    pub fn on_audio(&mut self, on_audio: Box<FnAudio>) {
        use self::ll::toxav_register_audio_callback as reg_cb;
        unsafe {
            let closure = box on_audio;
            let data: *mut c_void = mem::transmute(&*closure);
            
            reg_cb(self.av, Some(on_audio_callback), data);
            self.on_audio = Some(closure);
        }
    }

    pub fn group_audio(&mut self, oga: Box<FnGroupAudio>) -> GroupAudio {
        assert!(self.on_group_audio.is_none(), "`group_audio` can be called only once");
        let closure = box oga;
        let res = GroupAudio {
            closure: unsafe { mem::transmute(&*closure) },
            poison: self.poison.clone(),
        };
        self.on_group_audio = Some(closure);
        res
    }

    pub fn tick(&mut self) {
        unsafe {
            ll::toxav_do(self.av);
        }
    }

    pub fn wait(&mut self) {
        let time = unsafe { ll::toxav_do_interval(self.av) };
        timer::sleep(Duration::milliseconds(time as i64));
    }

    pub fn call(&self, friend_id: i32, settings: &CallSettings, rsec: i32) -> Result<i32, AvError> {
        let mut cid = 0i32;
        unsafe {
            av_result!(
                ll::toxav_call(self.av, (&mut cid as *mut _), friend_id, (settings as *const _), rsec),
                cid
            )
        }
    }

    pub fn hangup(&self, call_id: i32) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_hangup(self.av, call_id)) }
    }

    pub fn answer(&self, call_id: i32, settings: &CallSettings) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_answer(self.av, call_id, settings as *const _)) }
    }

    pub fn reject(&self, call_id: i32) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_reject(self.av, call_id, &0 as *const _)) }
    }

    pub fn cancel(&self, call_id: i32) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_cancel(self.av, call_id, 0, &0 as *const _)) }
    }

    pub fn change_settings(&self, call_id: i32, settings: &CallSettings) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_change_settings(self.av, call_id, settings as *const _)) }
    }

    pub fn transmission(&self, call_id: i32) -> Result<Transmittion, AvError> {
        unsafe {
            let res = ll::toxav_prepare_transmission(self.av, call_id, 0);
            av_result!(res).map(|_| Transmittion {av: &self, call_id: call_id})
        }
    }

    pub fn get_peer_id(&self, call_id: i32) -> Option<i32> {
        unsafe {
            let res = ll::toxav_get_peer_id(self.av, call_id, 0);
            match res {
                -1 => None,
                n => Some(n),
            }
        }
    }

    pub fn get_peer_settings(&self, call_id: i32,) -> Option<CallSettings> {
        unsafe {
            let mut settings: CallSettings = mem::uninitialized();
            let res =
                ll::toxav_get_peer_csettings(self.av, call_id, 0, &mut settings as *mut _);
            av_result!(res).map(|_| settings).ok()
        }
    }

    pub fn get_call_state(&self, call_id: i32) -> CallState {
        unsafe { ll::toxav_get_call_state(self.av, call_id) }
    }

    pub fn capability_supported(&self, call_id: i32, cap: Capability) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_capability_supported(self.av, call_id, cap)) }
    }

    pub fn get_active_count(&self) -> Option<i32> {
        unsafe {
            match ll::toxav_get_active_count(self.av) {
                -1 => None,
                n => Some(n as i32),
            }
        }
    }

    #[inline]
    unsafe fn from_raw_av(raw: *mut ll::ToxAv) -> ToxAv {
        let mut av: ToxAv = mem::zeroed();
        av.av = raw;
        av
    }
}

extern fn on_audio_callback(agent: *mut c_void, call_id: i32, pcm: *const i16, size: u16, data: *mut c_void) {
    unsafe {
        let closure: &mut FnAudio =
            mem::transmute_copy(mem::transmute::<_, &mut raw::Closure>(data));
        let pcm = slice::from_raw_parts(pcm, size as usize);
        let av = ToxAv::from_raw_av(agent as *mut ll::ToxAv);
        (*closure)(&av, call_id, pcm);
        mem::forget(av);
    }
}

macro_rules! event_callback {
    ($name: ident, $event: ident) => {
        extern fn $name(agent: *mut c_void, call_id: i32, arg: *mut c_void) {
            unsafe {
                let ev = AvEvent::$event(call_id);
                let closure: &mut FnEvent =
                    mem::transmute_copy(mem::transmute::<_, &mut raw::Closure>(arg));
                let av = ToxAv::from_raw_av(agent as *mut ll::ToxAv);
                (*closure)(&av, ev);
                mem::forget(av);
            }
        }
    }
}

event_callback!(on_invite, Invite);
event_callback!(on_ringing, Ringing);
event_callback!(on_start, Start);
event_callback!(on_cancel, Cancel);
event_callback!(on_reject, Reject);
event_callback!(on_end, End);
event_callback!(on_request_timeout, RequestTimeout);
event_callback!(on_peer_timeout, PeerTimeout);
event_callback!(on_peer_cs_change, PeerCsChange);
event_callback!(on_self_cs_change, SelfCsChange);
