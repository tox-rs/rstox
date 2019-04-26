#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InitError {
    // `NoError` doesn't exists in rust code
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    MallocError,
    PortAllocError,
    ProxyBadType,
    ProxyBadHost,
    ProxyBadPort,
    ProxyNotFound,
    LoadEncrypted,
    LoadBadFormat,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BootstrapError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    BadHost,
    BadPort,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetInfoError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    TooLong,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FriendAddError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    TooLong,
    NoMessage,
    OwnKey,
    AlreadySent,
    BadChecksum,
    SetNewNospam,
    MallocError,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FriendSendMessageError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    NotFound,
    NotConnected,
    SendQ,
    TooLong,
    Empty,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileControlError {
    #[doc(hidden)] NoError = 0,
    FriendNotFound = 1,
    FriendNotConnected,
    NotFound,
    NotPaused,
    Denied,
    AlreadyPaused,
    SendQ,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileSeekError {
    #[doc(hidden)] NoError = 0,
    FriendNotFound = 1,
    FriendNotConnected,
    NotFound,
    Denied,
    InvalidPosition,
    SendQ,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileGetError {
    #[doc(hidden)] NoError = 0,
    FriendNotFound = 1,
    NotFound,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileSendError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    FriendNotFound,
    FriendNotConnected,
    NameTooLong,
    TooMany,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileSendChunkError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    FriendNotFound,
    FriendNotConnected,
    NotFound,
    NotTransferring,
    InvalidLength,
    SendQ,
    WrongPosition,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FriendCustomPacketError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    FriendNotFound,
    FriendNotConnected,
    Invalid,
    Empty,
    TooLong,
    SendQ,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConferencePeerQueryError {
    #[doc(hidden)] NoError = 0,
    ConferenceNotFound = 1,
    PeerNotFound = 2,
    PeerQueryNoConnection = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConferenceInviteError {
    #[doc(hidden)] NoError = 0,
    ConferenceNotFound = 1,
    FailSend = 2,
    NoConnection = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConferenceJoinError {
    #[doc(hidden)] NoError = 0,
    InvalidLength = 1,
    WrongType = 2,
    FriendNotFound = 3,
    Duplicate = 4,
    InitFail = 5,
    FailSend = 6,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConferenceSendError {
    #[doc(hidden)] NoError = 0,
    ConferenceNotFound = 1,
    TooLong = 2,
    NoConnection = 3,
    FailSend = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConferenceTitleError {
    #[doc(hidden)] NoError = 0,
    ConferenceNotFound = 1,
    InvalidLength = 2,
    FailSend = 3,
}
