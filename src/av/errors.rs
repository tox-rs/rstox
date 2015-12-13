use std::error::Error;
use std::fmt;

///////////////////////////////
// Creation and destruction //
/////////////////////////////
/// Creation and destruction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_ERR_NEW {
    /// The function returned successfully.
    TOXAV_ERR_NEW_OK = 0,
    /// One of the arguments to the function was NULL when it was not expected.
    TOXAV_ERR_NEW_NULL,
    /// Memory allocation failure while trying to allocate structures required
    /// for the A/V session.
    TOXAV_ERR_NEW_MALLOC,
    /// Attempted to create a second session for the same Tox instance.
    TOXAV_ERR_NEW_MULTIPLE,
}

impl Error for TOXAV_ERR_NEW {
    fn description(&self) -> &str {
        use self::TOXAV_ERR_NEW::*;
        match *self {
            TOXAV_ERR_NEW_OK => "new: no error",
            TOXAV_ERR_NEW_NULL => "new: null",
            TOXAV_ERR_NEW_MALLOC => "new: failed to allocate memory",
            TOXAV_ERR_NEW_MULTIPLE =>
                "new: attempted to create a second session for same Tox instance",
        }
    }
}

impl fmt::Display for TOXAV_ERR_NEW {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


/////////////////
// Call setup //
///////////////
/// Call setup
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_ERR_CALL {
    /// The function returned successfully.
    TOXAV_ERR_CALL_OK = 0,
    /// A resource allocation error occured while trying to create the structures
    /// required for the call.
    TOXAV_ERR_CALL_MALLOC,
    /// Synchronization error occured.
    TOXAV_ERR_CALL_SYNC,
    /// The friend number did not designate a valid friend.
    TOXAV_ERR_CALL_FRIEND_NOT_FOUND,
    /// The friend was valid, but not currently connected.
    TOXAV_ERR_CALL_FRIEND_NOT_CONNECTED,
    /// Attempted to call a friend while already in an audio or video call with
    /// them.
    TOXAV_ERR_CALL_FRIEND_ALREADY_IN_CALL,
    /// Audio or video bit rate is invalid.
    TOXAV_ERR_CALL_INVALID_BIT_RATE,
}

impl Error for TOXAV_ERR_CALL {
    fn description(&self) -> &str {
        use self::TOXAV_ERR_CALL::*;
        match *self {
            TOXAV_ERR_CALL_OK => "call: no error",
            TOXAV_ERR_CALL_MALLOC => "call: failed to allocate memory",
            TOXAV_ERR_CALL_SYNC => "call: synchronization error ocurred",
            TOXAV_ERR_CALL_FRIEND_NOT_FOUND => "call: no friend with given friend number",
            TOXAV_ERR_CALL_FRIEND_NOT_CONNECTED => "call: friend is not connected",
            TOXAV_ERR_CALL_FRIEND_ALREADY_IN_CALL => "call: aready in call with friend",
            TOXAV_ERR_CALL_INVALID_BIT_RATE => "call: invalid bit rate",
        }
    }
}

impl fmt::Display for TOXAV_ERR_CALL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}



//////////////////
// Call answer //
////////////////
/// Call answer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_ERR_ANSWER {
    /// The function returned successfully.
    TOXAV_ERR_ANSWER_OK = 0,

    /// Synchronization error occurred.
    TOXAV_ERR_ANSWER_SYNC,
    /**
     * Failed to initialize codecs for call session. Note that codec initiation
     * will fail if there is no receive callback registered for either audio or
     * video.
     */
    TOXAV_ERR_ANSWER_CODEC_INITIALIZATION,
    /// The friend number did not designate valid friend.
    TOXAV_ERR_ANSWER_FRIEND_NOT_FROUND,
    /// The friend was valid, but they are not currently trying to initiate
    /// a call. This is also returned if this client is already in a call with
    /// the friend.
    TOXAV_ERR_ANSWER_FRIEND_NOT_CALLING,
    /// Audio or video bit rate is invalid.
    TOXAV_ERR_ANSWER_INVALID_BIT_RATE,
}

impl Error for TOXAV_ERR_ANSWER {
    fn description(&self) -> &str {
        use self::TOXAV_ERR_ANSWER::*;
        match *self {
            TOXAV_ERR_ANSWER_OK => "answer: no error",
            TOXAV_ERR_ANSWER_SYNC => "answer: synchronization error ocurred",
            TOXAV_ERR_ANSWER_CODEC_INITIALIZATION =>
                "answer: failed to initialize codec for session",
            TOXAV_ERR_ANSWER_FRIEND_NOT_FROUND =>
                "answer: no friend with given friend number",
            TOXAV_ERR_ANSWER_FRIEND_NOT_CALLING =>
                "answer: friend not calling or already in call",
            TOXAV_ERR_ANSWER_INVALID_BIT_RATE => "answer: invalid bit rate",
        }
    }
}

impl fmt::Display for TOXAV_ERR_ANSWER {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}



///////////////////////
// Call state graph //
/////////////////////
/// Call state graph
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_FRIEND_CALL_STATE {
    /**
     * Set by the AV core if an error occurred on the remote end or if friend
     * timed out. This is the final state after which no more state
     * transitions can occur for the call. This call state will never be
     * triggered in combination with other call states.
     */
    TOXAV_FRIEND_CALL_STATE_ERROR = 1,

    /// The call has finished. This is the final state after which no more state
    /// transitions can occur for the call. This call state will never be
    /// triggered in combination with other call states.
    TOXAV_FRIEND_CALL_STATE_FINISHED = 2,

    /// The flag that marks that friend is sending audio.
    TOXAV_FRIEND_CALL_STATE_SENDING_A = 4,

    /// The flag that marks that friend is sending video.
    TOXAV_FRIEND_CALL_STATE_SENDING_V = 8,

    /// The flag that marks that friend is receiving audio.
    TOXAV_FRIEND_CALL_STATE_ACCEPTING_A = 16,

    /// The flag that marks that friend is receiving video.
    TOXAV_FRIEND_CALL_STATE_ACCEPTING_V = 32,
}


///////////////////
// Call control //
/////////////////
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_ERR_CALL_CONTROL {
    /// The function returned successfully.
    TOXAV_ERR_CALL_CONTROL_OK = 0,

    /// Synchronization error occured.
    TOXAV_ERR_CALL_CONTROL_SYNC,

    /// The friend_number passed did not designate a valid friend.
    TOXAV_ERR_CALL_CONTROL_FRIEND_NOT_FOUND,

    /// This client is currently not in a call with the friend. Before the call
    /// is answered, only CANCEL is a valid control.
    TOXAV_ERR_CALL_CONTROL_FRIEND_NOT_IN_CALL,

    /// Happnes if user tried to pause an already paused call or if trying to
    /// resume a call that is not paused.
    TOXAV_ERR_CALL_CONTROL_INVALID_TRANSITION,
}

impl Error for TOXAV_ERR_CALL_CONTROL {
    fn description(&self) -> &str {
        use self::TOXAV_ERR_CALL_CONTROL::*;
        match *self {
            TOXAV_ERR_CALL_CONTROL_OK => "call_control: no error",
            TOXAV_ERR_CALL_CONTROL_SYNC =>
                "call_control: synchronization error ocurred",
            TOXAV_ERR_CALL_CONTROL_FRIEND_NOT_FOUND =>
                "call_control: no friend with given friend number",
            TOXAV_ERR_CALL_CONTROL_FRIEND_NOT_IN_CALL =>
                "call_control: not in call with friend",
            TOXAV_ERR_CALL_CONTROL_INVALID_TRANSITION =>
                "call_control: already paused or resumed",
        }
    }
}

impl fmt::Display for TOXAV_ERR_CALL_CONTROL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}



////////////////////////////
// Controlling bit rates //
//////////////////////////
/// Controlling bit rates
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_ERR_BIT_RATE_SET {
    /// The function returned successfully.
    TOXAV_ERR_BIT_RATE_SET_OK = 0,
    /// Synchronization error occurred.
    TOXAV_ERR_BIT_RATE_SET_SYNC,
    /// The audio bit rate passed was not one of the supported values.
    TOXAV_ERR_BIT_RATE_SET_INVALID_AUDIO_BIT_RATE,
    /// The video bit rate passed was not one of the supported values.
    TOXAV_ERR_BIT_RATE_SET_INVALID_VIDEO_BIT_RATE,
    /// The `friend_number` passed did not designate a valid friend.
    TOXAV_ERR_BIT_RATE_SET_FRIEND_NOT_FOUND,
    /// This client is currently not in a call with the friend.
    TOXAV_ERR_BIT_RATE_SET_FRIEND_NOT_IN_CALL,
}

impl Error for TOXAV_ERR_BIT_RATE_SET {
    fn description(&self) -> &str {
        use self::TOXAV_ERR_BIT_RATE_SET::*;
        match *self {
            TOXAV_ERR_BIT_RATE_SET_OK => "bit_rate: no error",
            TOXAV_ERR_BIT_RATE_SET_SYNC => "bit_rate: synchronization error ocurred",
            TOXAV_ERR_BIT_RATE_SET_INVALID_AUDIO_BIT_RATE =>
                "bit_rate: audio bit rate not supported",
            TOXAV_ERR_BIT_RATE_SET_INVALID_VIDEO_BIT_RATE =>
                "bit_rate: audio bit rate not supported",
            TOXAV_ERR_BIT_RATE_SET_FRIEND_NOT_FOUND =>
                "bit_rate: no friend with given friend number",
            TOXAV_ERR_BIT_RATE_SET_FRIEND_NOT_IN_CALL =>
                "bit_rate: not in call with friend",
        }
    }
}

impl fmt::Display for TOXAV_ERR_BIT_RATE_SET {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


//////////////////
// A/V sending //
////////////////
/// A/V sending
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TOXAV_ERR_SEND_FRAME {
    /// The function returned successfully.
    TOXAV_ERR_SEND_FRAME_OK = 0,
    /// In case of video, one of Y, U, or V was NULL. In case of audio, the
    /// samples data pointer was NULL. ← FIXME
    TOXAV_ERR_SEND_FRAME_NULL,
    /// The `friend_number` passed did not sedignate a valid friend.
    TOXAV_ERR_SEND_FRAME_FRIEND_NOT_FOUND,
    /// This client is currently not in a call with the friend.
    TOXAV_ERR_SEND_FRAME_FRIEND_NOT_IN_CALL,
    /// Synchronization error occurred.
    TOXAV_ERR_SEND_FRAME_SYNC,
    /// One of the frame parameters was invalid. E.g. the resolution may be too
    /// small or too large, or the audio sampling rate may be unsupported.
    TOXAV_ERR_SEND_FRAME_INVALID,
    /// Either friend turned off audio or video receiving or we turned off
    /// sending for the said payload.
    TOXAV_ERR_SEND_FRAME_PAYLOAD_TYPE_DISABLED,
    /// Failed to push frame through rtp interface.
    TOXAV_ERR_SEND_FRAME_RTP_FAILED,
}

impl Error for TOXAV_ERR_SEND_FRAME {
    fn description(&self) -> &str {
        use self::TOXAV_ERR_SEND_FRAME::*;
        match *self {
            TOXAV_ERR_SEND_FRAME_OK => "send_frame: no error",
            TOXAV_ERR_SEND_FRAME_NULL =>
                "send_frame: one of parameters was null", // FIXME?
            TOXAV_ERR_SEND_FRAME_FRIEND_NOT_FOUND =>
                "send_frame: no friend with given friend number",
            TOXAV_ERR_SEND_FRAME_FRIEND_NOT_IN_CALL =>
                "send_frame: not in call with friend",
            TOXAV_ERR_SEND_FRAME_SYNC =>
                "send_frame: synchronization error occured",
            TOXAV_ERR_SEND_FRAME_INVALID =>
                "send_frame: one of parameters was invalid",
            TOXAV_ERR_SEND_FRAME_PAYLOAD_TYPE_DISABLED =>
                "send_frame: either we or friend disabled this type of payload",
            TOXAV_ERR_SEND_FRAME_RTP_FAILED =>
                "send_frame: failed to push frame through rtp interface",
        }
    }
}

impl fmt::Display for TOXAV_ERR_SEND_FRAME {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
