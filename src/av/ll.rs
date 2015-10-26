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

//! **This is a low-level binding to `toxav`. Shouldn't be used directly - use
//! safe interface instead.**

use libc::{c_uint, c_void, size_t};
use super::{TOXAV_ERR_NEW, TOXAV_ERR_CALL, TOXAV_ERR_ANSWER, TOXAV_CALL_CONTROL};
use super::{TOXAV_ERR_CALL_CONTROL, TOXAV_ERR_BIT_RATE_SET, TOXAV_ERR_SEND_FRAME};
use core::Tox_Struct;

pub type Tox = Tox_Struct;


/**
 * The `ToxAV` instance type. (This is actually an opaque C struct - see
 * [Rust issue #27303](https://github.com/rust-lang/rust/issues/27303)
 * for more details on that.)
 *
 * Each `ToxAV` instance can be bound to only one
 * [`Tox`](../../core/struct.Tox.html) instance, and Tox instance can have
 * only one `ToxAV` instance. One must make sure to close `ToxAV` instance
 * prior to closing `Tox` instance otherwise undefined behaviour occurs. **Upon
 * closing of `ToxAV` instance, all active calls will be forcibly terminated
 * without notifying peers.**
 */
pub enum ToxAV {}




//////////////////////
/////////////////////
// Callback types //
///////////////////
//////////////////


/////////////////
// Call setup //
///////////////
/**
 * The function type for the [`call` callback](fn.toxav_callback_call.html).
 *
 * `friend_number` – The friend number from which the call is incoming.
 *
 * `audio_enabled` – True if friend is sending audio.
 *
 * `video_enabled` – True if friend is sending video.
 */
#[allow(non_camel_case_types)]
pub type toxav_call_cb =
    extern "C" fn(toxav: *mut ToxAV,
                  friend_number: u32,
                  audio_enabled: bool,
                  video_enabled: bool,
                  user_data: *mut c_void) -> ();


///////////////////////
// Call state graph //
/////////////////////
/**
 * The function type for the [`call_state`](fn.toxav_callback_call_state.html)
 * callback.
 *
 * `friend_number` – The friend number for which the call state changed.
 *
 * `state` – The bitmask of the new call state which is guaranteed to be
 * different than the previous state. The state is set to `0` when the call is
 * paused. The bitmask represents all the activities currently performed by the
 * friend.
 */
#[allow(non_camel_case_types)]
pub type toxav_call_state_cb =
    extern "C" fn(toxav: *mut ToxAV,
                  friend_number: u32,
                  state: u32,
                  user_data: *mut c_void) -> ();


////////////////////////////
// Controlling bit rates //
//////////////////////////
/**
 * The function type for the
 * [`bit_rate_status`](fn.toxav_callback_bit_rate_status.html) callback.
 *
 * The event is triggered when the network becomes too saturated for current
 * bit rates at which point core suggests new bit rates.
 *
 * `friend_number` – The friend number of the friend for which to set the
 * bit rate.
 *
 * `audio_bit_rate` – Suggested maximum audio bit rate in Kb/sec.
 *
 * `video_bit_rate` – Suggested maximum video bit rate in Kb/sec.
 */
#[allow(non_camel_case_types)]
pub type toxav_bit_rate_status_cb =
    extern "C" fn(toxav: *mut ToxAV,
                  friend_number: u32,
                  audio_bit_rate: u32,
                  video_bit_rate: u32,
                  user_data: *mut c_void) -> ();


////////////////////
// A/V receiving //
//////////////////
/**
 * The function type for the
 * [`audio_receive_frame`](fn.toxav_callback_audio_receive_frame.html) callback.
 *
 * The callback can be called multiple times per single iteration depending
 * on the amount of queued frames in the buffer. The received format is
 * the same as in send function.
 *
 * `friend_number` – The friend number of the friend who sent an audio frame.
 *
 * `pcm An array` – of audio samples (sample_count * channels elements).
 *
 * `sample_count` – The number of audio samples per channel in the PCM array.
 *
 * `channels` – Number of audio channels.
 *
 * `sampling_rate` – Sampling rate used in this frame.
 */
#[allow(non_camel_case_types)]
pub type toxav_audio_receive_frame_cb =
    extern "C" fn(toxav: *mut ToxAV,
                  friend_number: u32,
                  pcm: *mut i16,
                  sample_count: c_uint,
                  channels: u8,
                  sampling_rate: u32,
                  user_data: *mut c_void) -> ();

/**
 * The function type for the
 * [`video_receive_frame`](fn.toxav_callback_video_receive_frame.html) callback.
 *
 *  `friend_number The friend number of the friend who sent a video frame.
 *
 *  `width Width of the frame in pixels.
 *
 *  `height Height of the frame in pixels.
 *
 *  `y`
 *  `u`
 *  `v` Plane data.
 *        The size of plane data is derived from width and height where
 *        `Y = MAX(width, abs(ystride)) * height`,
 *        `U = MAX(width/2, abs(ustride)) * (height/2)` and
 *        `V = MAX(width/2, abs(vstride)) * (height/2)`.
 *  `ystride`
 *  `ustride`
 *  `vstride` – Strides data. Strides represent padding for each plane
 *                that may or may not be present. You must handle strides in
 *                your image processing code. Strides are negative if the
 *                image is bottom-up hence why you MUST `abs()` it when
 *                calculating plane buffer size.
 */
#[allow(non_camel_case_types)]
pub type toxav_video_receive_frame_cb =
    extern "C" fn(toxav: *mut ToxAV,
                  friend_number: u32,
                  width: u16,
                  height: u16,
                  y: *const u8,
                  u: *const u8,
                  v: *const u8,
                  ystride: i32,
                  ustride: i32,
                  vstride: i32,
                  user_data: *mut c_void) -> ();


#[link(name = "toxav")]
extern {
    ////////////////////////
    // ToxAV API Version //
    //////////////////////
    /**
     * Return the major version number of the library.
     *
     * Can be used to display the ToxAV library version or to check whether
     * the client is compatible with the dynamically linked version of ToxAV.
     */
    pub fn toxav_version_major() -> u32;

    /// Return the minor version number of the library.
    pub fn toxav_version_minor() -> u32;

    /// Return the patch number of the library.
    pub fn toxav_version_patch() -> u32;

    /// Return whether the compiled library version is compatible with the passed
    /// version numbers.
    pub fn toxav_version_is_compatible(major: u32, minor: u32, patch: u32) -> bool;


    ///////////////////////////////
    // Creation and destruction //
    /////////////////////////////
    /**
     * Start new A/V session. There can only be one session per
     * [`Tox`](../../core/struct.Tox.html) instance.
     */
    pub fn toxav_new(tox: *mut Tox, error: *mut TOXAV_ERR_NEW) -> ToxAV;

    /**
     * Releases all resources associated with the A/V session.
     *
     * If any calls were ongoing, these will be forcibly terminated without
     * notifying peers. After calling this function, no other functions may be
     * called and the av pointer becomes invalid.
     */
    pub fn toxav_kill(toxav: *mut ToxAV) -> ();



    /////////////////////
    // A/V event loop //
    ///////////////////

    /**
     * Returns the [`Tox`](../../core/struct.Tox.html) instance for which
     * the A/V object was created for.
     */
    pub fn toxav_get_tox(toxav: *const ToxAV) -> Tox;

    /**
     * Returns the interval in milliseconds when the next
     * [`toxav_iterate`](fn.toxav_iterate.html)
     * call should be.
     *
     * If no call is active at the moment, this function
     * returns 200.
     */
    pub fn toxav_iteration_interval(toxav: *const ToxAV) -> u32;

    /**
     * Main loop for the session.
     *
     * This function needs to be called in intervals of
     * [`toxav_iteration_interval()`](fn.toxav_iteration_interval.html)
     * milliseconds. It is best called in the separate thread from `tox_iterate`.
     */
    pub fn toxav_iterate(toxav: *mut ToxAV) -> ();




    /////////////////
    // Call setup //
    ///////////////
    /**
     * Call a friend. This will start ringing the friend.
     *
     * It is the client's responsibility to stop ringing after a certain timeout,
     * if such behaviour is desired. If the client does not stop ringing, the
     * library will not stop until the friend is disconnected. Audio and video
     * receiving are both enabled by default.
     *
     * `friend_number` –  The friend number of the friend that should be called.
     *
     * `audio_bit_rate` Audio bit rate in Kb/sec. Set this to `0` to disable
     * audio sending.
     *
     * `video_bit_rate` Video bit rate in Kb/sec. Set this to `0` to disable
     * video sending.
     */
    pub fn toxav_call(toxav: *mut ToxAV,
                      friend_number: u32,
                      audio_bit_rate: u32,
                      video_bit_rate: u32,
                      error: *mut TOXAV_ERR_CALL) -> bool;


    /// Set the callback for the [`call`](type.toxav_call_cb.html) event.
    ///
    /// Pass NULL to unset.
    pub fn toxav_callback_call(toxav: *mut ToxAV,
                               function: toxav_call_cb,
                               user_data: *mut c_void) -> ();

    /**
     * Accept an incoming call.
     *
     * If answering fails for any reason, the call will still be pending and it
     * is possible to try and answer it later. Audio and video receiving are
     * both enabled by default.
     *
     * `friend_number` – The friend number of the friend that is calling.
     *
     * `audio_bit_rate` – Audio bit rate in Kb/sec. Set this to 0 to disable
     * audio sending.
     *
     * `video_bit_rate` – Video bit rate in Kb/sec. Set this to 0 to disable
     * video sending.
     */
    pub fn toxav_answer(toxav: *mut ToxAV,
                        friend_number: u32,
                        audio_bit_rate: u32,
                        video_bit_rate: u32,
                        error: *mut TOXAV_ERR_ANSWER) -> bool;


    ///////////////////////
    // Call state graph //
    /////////////////////
    /// Set the callback for the [`call_state`](type.toxav_ event_cb.html) event.
    ///
    /// Pass NULL to unset.
    pub fn toxav_callback_call_state(toxav: *mut ToxAV,
                                     function: toxav_call_state_cb,
                                     user_data: *mut c_void) -> ();


    ///////////////////
    // Call control //
    /////////////////
    /**
     * Sends a call control command to a friend.
     *
     * `friend_number` – The friend number of the friend this client is in
     * a call with.
     *
     * `control` – The control command to send.
     */
    pub fn toxav_call_control(toxav: *mut ToxAV,
                              friend_number: u32,
                              control: TOXAV_CALL_CONTROL,
                              error: *mut TOXAV_ERR_CALL_CONTROL) -> bool;


    ////////////////////////////
    // Controlling bit rates //
    //////////////////////////
    /**
     * Set the bit rate to be used in subsequent audio/video frames.
     *
     * `friend_number` – The friend number of the friend for which to set the
     * bit rate.
     *
     * `audio_bit_rate` – The new audio bit rate in Kb/sec. Set to 0 to disable
     * audio sending. Set to -1 to leave unchanged.
     *
     * `video_bit_rate` – The new video bit rate in Kb/sec. Set to 0 to disable
     * video sending. Set to -1 to leave unchanged.
     *
     */
    pub fn toxav_bit_rate_set(toxav: *mut ToxAV,
                              friend_number: u32,
                              audio_bit_rate: i32,
                              video_bit_rate: i32,
                              error: *mut TOXAV_ERR_BIT_RATE_SET) -> bool;

    /// Set the callback for the
    /// [`bit_rate_status`](type.toxav_bit_rate_status_cb.html) event.
    ///
    /// Pass NULL to unset.
    pub fn toxav_callback_bit_rate_status(toxav: *mut ToxAV,
                                          function: toxav_bit_rate_status_cb,
                                          user_data: *mut c_void) -> ();



    //////////////////
    // A/V sending //
    ////////////////
    /**
     * Send an audio frame to a friend.
     *
     * The expected format of the PCM data is:
     * `[s1c1]``[s1c2]``[...]``[s2c1]``[s2c2]``[...]...`
     * Meaning: `sample 1` for `channel 1`, `sample 1` for `channel 2`, ...
     * For mono audio, this has no meaning, every sample is subsequent. For
     * stereo, this means the expected format is `LRLRLR...` with samples
     * for left and right alternating.
     *
     * `friend_number` – The friend number of the friend to which to send an
     * audio frame.
     *
     * `pcm` – An array of audio samples. The size of this array must be
     * sample_count * channels.
     *
     * `sample_count` – Number of samples in this frame. Valid numbers here are
     * `((sample rate) * (audio length) / 1000)`, where audio length can be
     * `2.5`, `5`, `10`, `20`, `40` or `60` millseconds.
     *
     * `channels` – Number of audio channels. Supported values are 1 and 2.
     *
     * `sampling_rate` – Audio sampling rate used in this frame. Valid sampling
     * rates are `8000`, `12000`, `16000`, `24000`, or `48000`.
     */
    pub fn toxav_audio_send_frame(toxav: *mut ToxAV,
                                  friend_number: u32,
                                  pcm: *const i16,
                                  sample_count: size_t,
                                  channels: u8,
                                  sampling_rate: u32,
                                  error: *mut TOXAV_ERR_SEND_FRAME) -> bool;

    /**
     * Send a video frame to a friend.
     *
     * `Y` - plane should be of size: `height * width`
     *
     * `U` - plane should be of size: `(height/2) * (width/2)`
     *
     * `V` - plane should be of size: `(height/2) * (width/2)`
     *
     * `friend_number` – The friend number of the friend to which to send a video
     * frame.
     *
     * `width` – Width of the frame in pixels.
     *
     * `height` – Height of the frame in pixels.
     *
     * `y` – Y (Luminance) plane data.
     *
     * `u` – U (Chroma) plane data.
     *
     * `v` – V (Chroma) plane data.
     */
    pub fn toxav_video_send_frame(toxav: *mut ToxAV,
                                  friend_number: u32,
                                  width: u16,
                                  height: u16,
                                  y: *const u8,
                                  u: *const u8,
                                  v: *const u8,
                                  error: *mut TOXAV_ERR_SEND_FRAME) -> bool;


    ////////////////////
    // A/V receiving //
    //////////////////
    /// Set the callback for the
    /// [`audio_receive_frame`](type.toxav_audio_receive_frame_cb.html) event.
    /// Pass NULL to unset.
    pub fn toxav_callback_audio_receive_frame(toxav: *mut ToxAV,
                                              function:
                                                  toxav_audio_receive_frame_cb,
                                              user_data: *mut c_void) -> ();

    /// Set the callback for the
    /// [`video_receive_frame`](type.toxav_video_receive_frame_cb.html) event.
    ///
    /// Pass NULL to unset.
    pub fn toxav_callback_video_receive_frame(toxav: *mut ToxAV,
                                              function:
                                                  toxav_video_receive_frame_cb,
                                              user_data: *mut c_void) -> ();


/////////////////////////////////////////////////////////
// NOTE: THERE IS NO SUPPORT FOR OLD AUDIO GROUPCHATS //
///////////////////////////////////////////////////////
}
