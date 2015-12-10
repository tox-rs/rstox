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
