/*
    Copyright © 2015 Сухарик <suhr@i2pmail.org>
    Copyright © 2015 Zetok Zalbavar <zexavexxe@gmail.com>


    This file is part of rstox.

    rstox is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    rstox is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with rstox.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Safe interface to `toxav`.


//use std::cell::RefCell;


use libc::{/*c_int,*/ c_uint, c_void};
use std::mem;
use std::sync::mpsc::Sender;

use core::{Tox, Event};

pub mod ll;
pub mod errors;

////////////////////////
// ToxAV API Version //
//////////////////////
/// Return the major version of the `toxav` library.
pub fn av_version_major() -> u32 {
    unsafe { ll::toxav_version_major() }
}

#[test]
// Current major version should equal to `0`
fn test_av_version_major() {
    assert_eq!(av_version_major(), 0);
}


/// Return the minor version of the `toxav` library.
pub fn av_version_minor() -> u32 {
    unsafe { ll::toxav_version_major() }
}

#[test]
// Current minor version should equal to `0`
fn test_av_version_minor() {
    assert_eq!(av_version_minor(), 0);
}


/// Return the patch version of the `toxav` library.
pub fn av_version_patch() -> u32 {
    unsafe { ll::toxav_version_patch() }
}

#[test]
// Current patch version should equal to `0`
fn test_av_version_patch() {
    assert_eq!(av_version_patch(), 0);
}


/// Return whether the compiled library version is compatible with the passed
/// version numbers. **Apparently until `toxcore` will get proper versions, it
/// will always return `true`.**
pub fn av_version_is_compatible(major: u32, minor: u32, patch: u32) -> bool {
    unsafe { ll::toxav_version_is_compatible(major, minor, patch) }
}

#[test]
// Current version numbers should be `0, 0, 0`
fn test_av_version_is_compatible() {
    assert_eq!(av_version_is_compatible(0, 0, 0), true);
    // apparently until toxcore gets proper versions it's always true
    // TODO: uncomment when it should work
    //assert_eq!(av_version_is_compatible(1, 1, 1), false);
    //assert_eq!(av_version_is_compatible(999999, 999999, 999999), false);
}



//////////////////////

macro_rules! tox_try {
    ($err:ident, $exp:expr) => {{
        let mut $err = ::std::mem::uninitialized();
        let res = $exp;
        match $err as c_uint {
            0 => {},
            _ => return Err($err),
        };
        res
    }};
}

//////////////////////

/// Call control
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_CALL_CONTROL {
    /// Resume a previously paused call. Only valid if the pause was caused by
    /// this client, if not, this control is ignored. Not valid before the call
    /// is accepted.
    TOXAV_CALL_CONTROL_RESUME = 0,

    /// Put a call on hold. Not valid before the call is accepted.
    TOXAV_CALL_CONTROL_PAUSE,

    /// Reject a call if it iwas not answered, yet. Cancel a call after it was
    /// answered.
    TOXAV_CALL_CONTROL_CANCEL,

    /// Request that the friend stops sending audio. Regardless of the friend's
    /// compilance, this will cause the `audio_receive_frame` event to stop
    /// being triggered on receiving an audio from from the friend.
    TOXAV_CALL_CONTROL_MUTE_AUDIO,

    /// Calling this control willl notify client to start sending audio again.
    TOXAV_CALL_CONTROL_UNMUTE_AUDIO,

    /// Request that the friend stops sending video. Regardless of the friend's
    /// compilance, this will cause the `video_receive_frame` event to stop
    /// being triggered on receiving a video frame from the friend.
    TOXAV_CALL_CONTROL_MUTE_VIDEO,

    /// Calling this control will notify client to start sending video again.
    TOXAV_CALL_CONTROL_UNMUTE_VIDEO,
}


///////////////////////////////
// Creation and destruction //
/////////////////////////////
pub struct ToxAv {
    av: *mut ll::ToxAV
}

impl ToxAv {
    pub fn new(tox: &mut Tox) -> Result<ToxAv, errors::TOXAV_ERR_NEW> {
        let mut toxav = ToxAv {
            av: unsafe { tox_try!(err, ll::toxav_new(tox.raw, &mut err)) }
        };
        toxav.init(tox);
        Ok(toxav)
    }

    fn init(&mut self, tox: &mut Tox) {
        unsafe {
            let chan: *mut c_void = mem::transmute(&mut *tox.event_tx);
            ll::toxav_callback_call(self.av, on_call, chan);
            ll::toxav_callback_call_state(self.av, on_call_state, chan);
            ll::toxav_callback_bit_rate_status(self.av, on_bit_rate_status, chan);
            ll::toxav_callback_audio_receive_frame(self.av, on_audio_receive_frame, chan);
            ll::toxav_callback_video_receive_frame(self.av, on_video_receive_frame, chan);
        }
    }

    pub fn interval(&self) -> u32 {
        unsafe { ll::toxav_iteration_interval(self.av) }
    }

    pub fn iterate(&mut self) {
        unsafe { ll::toxav_iterate(self.av) };
    }

    pub fn call(
        &mut self,
        friend_number: u32,
        audio_bitrate: u32,
        video_bitrate: u32
    ) -> Result<bool, errors::TOXAV_ERR_CALL> {
        Ok(unsafe {
            tox_try!(err, ll::toxav_call(
                self.av,
                friend_number,
                audio_bitrate,
                video_bitrate,
                &mut err
            ))
        })
    }

    pub fn answer(
        &mut self,
        friend_number: u32,
        audio_bitrate: u32,
        video_bitrate: u32
    ) -> Result<bool, errors::TOXAV_ERR_ANSWER> {
        Ok(unsafe {
            tox_try!(err, ll::toxav_answer(
                self.av,
                friend_number,
                audio_bitrate,
                video_bitrate,
                &mut err
            ))
        })
    }

    pub fn control(
        &mut self,
        friend_number: u32,
        control: TOXAV_CALL_CONTROL
    ) -> Result<bool, errors::TOXAV_ERR_CALL_CONTROL> {
        Ok(unsafe {
            tox_try!(err, ll::toxav_call_control(
                self.av,
                friend_number,
                control,
                &mut err
            ))
        })
    }

    pub fn set_bitrate(
        &mut self,
        friend_number: u32,
        audio_bitrate: i32,
        video_bitrate: i32,
    ) -> Result<bool, errors::TOXAV_ERR_BIT_RATE_SET> {
        Ok(unsafe {
            tox_try!(err, ll::toxav_bit_rate_set(
                self.av,
                friend_number,
                audio_bitrate,
                video_bitrate,
                &mut err
            ))
        })
    }
}

impl Drop for ToxAv {
    fn drop(&mut self) {
        unsafe { ll::toxav_kill(self.av); }
    }
}

extern fn on_call(
    toxav: *mut ll::ToxAV,
    friend_number: u32,
    audio_enabled: bool,
    video_enabled: bool,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        tx.send(Event::Call(friend_number, audio_enabled, video_enabled)).ok();
    }
}

extern fn on_call_state(
    toxav: *mut ll::ToxAV,
    friend_number: u32,
    state: u32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        tx.send(Event::CallState(friend_number, state)).ok();
    }
}

extern fn on_bit_rate_status(
    toxav: *mut ll::ToxAV,
    friend_number: u32,
    audio_bitrate: u32,
    video_bitrate: u32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        tx.send(Event::BitRateStatus(friend_number, audio_bitrate, video_bitrate)).ok();
    }
}

extern fn on_audio_receive_frame(
    toxav: *mut ll::ToxAV,
    friend_number: u32,
    pcm: *const i16,
    sample_count: u32,
    channels: u8,
    sampling_rate: u32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        tx.send(Event::AudioReceiveFrame(friend_number, *pcm, sample_count, channels, sampling_rate)).ok();
    }
}

extern fn on_video_receive_frame(
    toxav: *mut ll::ToxAV,
    friend_number: u32,
    width: u16,
    height: u16,
    y: *const u8,
    u: *const u8,
    v: *const u8,
    ystride: i32,
    ustride: i32,
    vstride: i32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        tx.send(Event::VideoReceiveFrame(friend_number, width, height, *y, *u, *v, ystride, ustride, vstride)).ok();
    }
}
