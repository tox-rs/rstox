use std::sync::Arc;
use std::cell::{RefCell, UnsafeCell};
use std::error::Error;
use std::fmt;
use core::{Tox};

mod ll;

#[repr(C)]
#[derive(Copy)]
pub enum CallType {
    Audio = 192,
    Video
}

#[repr(C)]
#[derive(Copy)]
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
#[derive(Copy)]
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
#[derive(Clone, Copy)]
pub enum Event {
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

// Audio, received in group audio callback
pub struct AudioBit<'a> {
    pub pcm: &'a [i16],
    pub samples: u32,
    pub channels: u8,
    pub sample_rate: u32,
}

pub type FnEvent = FnMut(&ToxAv, Event) + 'static;
pub type FnAudio = FnMut(&ToxAv, &[i16]) + 'static;
pub type FnGroupAudio = FnMut(&mut ToxAv, AudioBit) + 'static;

pub struct GroupAudio {
    closure: *mut FnGroupAudio,
    poison: Arc<UnsafeCell<bool>>, // mb use AtomicBool instead?
}

impl GroupAudio {
    pub fn new_groupchat(&self, tox: &mut Tox) -> Option<i32> {
        unimplemented!()
    }
    pub fn join_groupchat(&self, tox: &mut Tox) -> Option<i32> {
        unimplemented!()
    }
    pub fn send_audio(&self, tox: &mut Tox, bit: AudioBit) -> Result<(),()> {
        unimplemented!()
    }
}


pub struct ToxAv {
    av: *mut ll::ToxAv,
    tox: Arc<RefCell<Tox>>,
    on_event: Option<Box<FnEvent>>,
    on_audio: Option<Box<FnAudio>>,
    on_group_audio: Option<Box<FnGroupAudio>>,
    poison: Arc<UnsafeCell<bool>>,
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
        unimplemented!()
    }
    pub fn on_audio(&mut self, on_audio: Box<FnAudio>) {
        unimplemented!()
    }
    pub fn group_audio(&mut self, mut oga: Box<FnGroupAudio>) -> GroupAudio {
        assert!(self.on_group_audio.is_none());
/*        let res = GroupAudio {
            closure: &mut *oga as *mut FnGroupAudio,
            poison: self.poison.clone(),
        };
        self.on_group_audio = Some(oga);
        res*/
        unimplemented!()
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
    pub fn cancel(&self, call_id: i32, peer: i32) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_cancel(self.av, call_id, peer, &0 as *const _)) }
    }
    pub fn change_settings(&self, call_id: i32, settings: &CallSettings) -> Result<(), AvError> {
        unsafe { av_result!(ll::toxav_change_settings(self.av, call_id, settings as *const _)) }
    }
    // Transmittion stuff will be here
    // BEGIN
    // END
    //pub fn get_peer_id() -> i32
    //pub fn get_peer_settings(&self, call_id: i32, peer: i32,)
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
                c => Some(c),
            }
        }
    }
}
