use std::error::Error;
use std::fmt;

///////////////////////////////
// Creation and destruction //
/////////////////////////////
/// Creation and destruction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum NewError {
    /// The function returned successfully.
    #[doc(hidden)] NoError = 0,
    /// One of the arguments to the function was NULL when it was not expected.
    NullError = 1,
    /// Memory allocation failure while trying to allocate structures required
    /// for the A/V session.
    MallocError,
    /// Attempted to create a second session for the same Tox instance.
    Multiple,
}

impl Error for NewError {
    fn description(&self) -> &str {
        match *self {
            NewError::NoError => "new: no error",
            NewError::NullError => "new: null",
            NewError::MallocError => "new: failed to allocate memory",
            NewError::Multiple =>
                "new: attempted to create a second session for same Tox instance",
        }
    }
}

impl fmt::Display for NewError {
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
pub enum CallError {
    /// The function returned successfully.
    #[doc(hidden)] NoError = 0,
    /// A resource allocation error occured while trying to create the structures
    /// required for the call.
    MallocError,
    /// Synchronization error occured.
    SyncError,
    /// The friend number did not designate a valid friend.
    FriendNotFound,
    /// The friend was valid, but not currently connected.
    FriendNotConnected,
    /// Attempted to call a friend while already in an audio or video call with
    /// them.
    FriendAlreadyInCall,
    /// Audio or video bit rate is invalid.
    InvalidBitRate
}

impl Error for CallError {
    fn description(&self) -> &str {
        match *self {
            CallError::NoError => "call: no error",
            CallError::MallocError => "call: failed to allocate memory",
            CallError::SyncError => "call: synchronization error ocurred",
            CallError::FriendNotFound => "call: no friend with given friend number",
            CallError::FriendNotConnected => "call: friend is not connected",
            CallError::FriendAlreadyInCall => "call: aready in call with friend",
            CallError::InvalidBitRate => "call: invalid bit rate",
        }
    }
}

impl fmt::Display for CallError {
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
pub enum AnswerError {
    /// The function returned successfully.
    #[doc(hidden)] NoError = 0,

    /// Synchronization error occurred.
    SyncError,
    /**
     * Failed to initialize codecs for call session. Note that codec initiation
     * will fail if there is no receive callback registered for either audio or
     * video.
     */
    CodecInitializationError,
    /// The friend number did not designate valid friend.
    FriendNotFound,
    /// The friend was valid, but they are not currently trying to initiate
    /// a call. This is also returned if this client is already in a call with
    /// the friend.
    FriendNotCalling,
    /// Audio or video bit rate is invalid.
    InvalidBitRate
}

impl Error for AnswerError {
    fn description(&self) -> &str {
        match *self {
            AnswerError::NoError => "answer: no error",
            AnswerError::SyncError => "answer: synchronization error ocurred",
            AnswerError::CodecInitializationError =>
                "answer: failed to initialize codec for session",
            AnswerError::FriendNotFound =>
                "answer: no friend with given friend number",
            AnswerError::FriendNotCalling =>
                "answer: friend not calling or already in call",
            AnswerError::InvalidBitRate => "answer: invalid bit rate",
        }
    }
}

impl fmt::Display for AnswerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


///////////////////
// Call control //
/////////////////
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CallControlError {
    /// The function returned successfully.
    #[doc(hidden)] NoError = 0,

    /// Synchronization error occured.
    SyncError,

    /// The friend_number passed did not designate a valid friend.
    FriendNotFound,

    /// This client is currently not in a call with the friend. Before the call
    /// is answered, only CANCEL is a valid control.
    FriendNotInCall,

    /// Happnes if user tried to pause an already paused call or if trying to
    /// resume a call that is not paused.
    InvalidTransition
}

impl Error for CallControlError {
    fn description(&self) -> &str {
        match *self {
            CallControlError::NoError => "call_control: no error",
            CallControlError::SyncError =>
                "call_control: synchronization error ocurred",
            CallControlError::FriendNotFound =>
                "call_control: no friend with given friend number",
            CallControlError::FriendNotInCall =>
                "call_control: not in call with friend",
            CallControlError::InvalidTransition =>
                "call_control: already paused or resumed",
        }
    }
}

impl fmt::Display for CallControlError {
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
pub enum BitRateSetError {
    /// The function returned successfully.
    #[doc(hidden)] NoError = 0,
    /// Synchronization error occurred.
    SyncError,
    /// The audio bit rate passed was not one of the supported values.
    InvalidAudioBitRate,
    /// The video bit rate passed was not one of the supported values.
    InvalidVideoBitRate,
    /// The `friend_number` passed did not designate a valid friend.
    FriendNotFound,
    /// This client is currently not in a call with the friend.
    FriendNotInCall
}

impl Error for BitRateSetError {
    fn description(&self) -> &str {
        match *self {
            BitRateSetError::NoError => "bit_rate: no error",
            BitRateSetError::SyncError => "bit_rate: synchronization error ocurred",
            BitRateSetError::InvalidAudioBitRate =>
                "bit_rate: audio bit rate not supported",
            BitRateSetError::InvalidVideoBitRate =>
                "bit_rate: video bit rate not supported",
            BitRateSetError::FriendNotFound =>
                "bit_rate: no friend with given friend number",
            BitRateSetError::FriendNotInCall =>
                "bit_rate: not in call with friend",
        }
    }
}

impl fmt::Display for BitRateSetError {
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
pub enum SendFrameError {
    /// The function returned successfully.
    #[doc(hidden)] NoError = 0,
    /// In case of video, one of Y, U, or V was NULL. In case of audio, the
    /// samples data pointer was NULL. ← FIXME
    NullError = 1,
    /// The `friend_number` passed did not sedignate a valid friend.
    FriendNotFound,
    /// This client is currently not in a call with the friend.
    FriendNotInCall,
    /// Synchronization error occurred.
    SyncError,
    /// One of the frame parameters was invalid. E.g. the resolution may be too
    /// small or too large, or the audio sampling rate may be unsupported.
    Invalid,
    /// Either friend turned off audio or video receiving or we turned off
    /// sending for the said payload.
    PayloadTypeDisabled,
    /// Failed to push frame through rtp interface.
    RtpFailed
}

impl Error for SendFrameError {
    fn description(&self) -> &str {
        match *self {
            SendFrameError::NoError => "send_frame: no error",
            SendFrameError::NullError =>
                "send_frame: one of parameters was null", // FIXME?
            SendFrameError::FriendNotFound =>
                "send_frame: no friend with given friend number",
            SendFrameError::FriendNotInCall =>
                "send_frame: not in call with friend",
            SendFrameError::SyncError =>
                "send_frame: synchronization error occured",
            SendFrameError::Invalid =>
                "send_frame: one of parameters was invalid",
            SendFrameError::PayloadTypeDisabled =>
                "send_frame: either we or friend disabled this type of payload",
            SendFrameError::RtpFailed =>
                "send_frame: failed to push frame through rtp interface",
        }
    }
}

impl fmt::Display for SendFrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
