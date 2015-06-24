
//use libc::{c_int, c_uint, c_char, c_void};
use super::{CallSettings, CallState, Capability};
use core::{Tox_Struct};

pub type Tox = Tox_Struct;

#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct ToxAv;

pub type ToxAVCallback =
    ::std::option::Option<extern "C" fn
                              (agent: *mut ::libc::c_void, call_idx: i32,
                               arg: *mut ::libc::c_void)>;
pub type ToxAvAudioCallback =
    ::std::option::Option<extern "C" fn
                              (agent: *mut ::libc::c_void, call_idx: i32,
                               PCM: *const i16, size: u16,
                               data: *mut ::libc::c_void)>;
/*
  pub type ToxAvVideoCallback =
    ::std::option::Option<extern "C" fn
                              (agent: *mut ::libc::c_void, call_idx: i32,
                               img: *const vpx_image_t,
                               data: *mut ::libc::c_void)>;
*/
#[repr(C)]
#[derive(Clone, Copy)]
pub enum ToxAvCallbackID {
  OnInvite = 0,
  OnRinging,
  OnStart,
  OnCancel,
  OnReject,
  OnEnd,
  OnRequestTimeout,
  OnPeerTimeout,
  OnPeerCSChange,
  OnSelfCSChange,
}

#[link(name = "toxav")]
extern {
    pub fn toxav_new(messenger: *mut Tox, max_calls: i32) -> *mut ToxAv;
    pub fn toxav_kill(av: *mut ToxAv);
    pub fn toxav_do_interval(av: *mut ToxAv) -> u32;
    pub fn toxav_do(av: *mut ToxAv);
    pub fn toxav_register_callstate_callback(av: *mut ToxAv,
                                             cb: ToxAVCallback,
                                             id: ToxAvCallbackID,
                                             userdata: *mut ::libc::c_void);
    pub fn toxav_register_audio_callback(av: *mut ToxAv,
                                         cb: ToxAvAudioCallback,
                                         userdata: *mut ::libc::c_void);
/*    pub fn toxav_register_video_callback(av: *mut ToxAv,
                                         cb: ToxAvVideoCallback,
                                         userdata: *mut ::libc::c_void);*/
    pub fn toxav_call(av: *mut ToxAv, call_index: *mut i32,
                      friend_id: ::libc::c_int,
                      csettings: *const CallSettings,
                      ringing_seconds: ::libc::c_int) -> ::libc::c_int;
    pub fn toxav_hangup(av: *mut ToxAv, call_index: i32) -> ::libc::c_int;
    pub fn toxav_answer(av: *mut ToxAv, call_index: i32,
                        csettings: *const CallSettings) -> ::libc::c_int;
    pub fn toxav_reject(av: *mut ToxAv, call_index: i32,
                        reason: *const ::libc::c_char) -> ::libc::c_int;
    pub fn toxav_cancel(av: *mut ToxAv, call_index: i32,
                        peer_id: ::libc::c_int, reason: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn toxav_change_settings(av: *mut ToxAv, call_index: i32,
                                 csettings: *const CallSettings)
     -> ::libc::c_int;
    pub fn toxav_stop_call(av: *mut ToxAv, call_index: i32)
     -> ::libc::c_int;
    pub fn toxav_prepare_transmission(av: *mut ToxAv, call_index: i32,
                                      support_video: ::libc::c_int)
     -> ::libc::c_int;
    pub fn toxav_kill_transmission(av: *mut ToxAv, call_index: i32)
     -> ::libc::c_int;
/*
    pub fn toxav_prepare_video_frame(av: *mut ToxAv, call_index: i32,
                                     dest: *mut u8,
                                     dest_max: ::libc::c_int,
                                     input: *mut vpx_image_t) 
     -> ::libc::c_int;
    pub fn toxav_send_video(av: *mut ToxAv, call_index: i32,
                            frame: *const u8, frame_size: u32)
     -> ::libc::c_int;
*/
    pub fn toxav_prepare_audio_frame(av: *mut ToxAv, call_index: i32,
                                     dest: *mut u8,
                                     dest_max: ::libc::c_int,
                                     frame: *const i16,
                                     frame_size: ::libc::c_int)
     -> ::libc::c_int;
    pub fn toxav_send_audio(av: *mut ToxAv, call_index: i32,
                            frame: *const u8, size: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn toxav_get_peer_csettings(av: *mut ToxAv, call_index: i32,
                                    peer: ::libc::c_int,
                                    dest: *mut CallSettings)
     -> ::libc::c_int;
    pub fn toxav_get_peer_id(av: *mut ToxAv, call_index: i32,
                             peer: ::libc::c_int) -> ::libc::c_int;
    pub fn toxav_get_call_state(av: *mut ToxAv, call_index: i32)
     -> CallState;
    pub fn toxav_capability_supported(av: *mut ToxAv, call_index: i32,
                                      capability: Capability)
     -> ::libc::c_int;
    /*pub fn toxav_get_tox(av: *mut ToxAv) -> *mut Tox;*/
    pub fn toxav_get_active_count(av: *mut ToxAv) -> ::libc::c_int;
    pub fn toxav_add_av_groupchat(tox: *mut Tox,
                                  audio_callback:
                                      ::std::option::Option<extern "C" fn
                                                                (arg1:
                                                                     *mut Tox,
                                                                 arg2:
                                                                     ::libc::c_int,
                                                                 arg3:
                                                                     ::libc::c_int,
                                                                 arg4:
                                                                     *const i16,
                                                                 arg5:
                                                                     ::libc::c_uint,
                                                                 arg6:
                                                                     u8,
                                                                 arg7:
                                                                     ::libc::c_uint,
                                                                 arg8:
                                                                     *mut ::libc::c_void)>,
                                  userdata: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn toxav_join_av_groupchat(tox: *mut Tox, friendnumber: i32,
                                   data: *const u8, length: u16,
                                   audio_callback:
                                       ::std::option::Option<extern "C" fn
                                                                 (arg1:
                                                                      *mut Tox,
                                                                  arg2:
                                                                      ::libc::c_int,
                                                                  arg3:
                                                                      ::libc::c_int,
                                                                  arg4:
                                                                      *const i16,
                                                                  arg5:
                                                                      ::libc::c_uint,
                                                                  arg6:
                                                                      u8,
                                                                  arg7:
                                                                      ::libc::c_uint,
                                                                  arg8:
                                                                      *mut ::libc::c_void)>,
                                   userdata: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn toxav_group_send_audio(tox: *mut Tox, groupnumber: ::libc::c_int,
                                  pcm: *const i16,
                                  samples: ::libc::c_uint, channels: u8,
                                  sample_rate: ::libc::c_uint)
     -> ::libc::c_int;
}
