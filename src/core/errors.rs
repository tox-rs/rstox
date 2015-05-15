

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InitError {
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
    NullError = 1,
    BadHost,
    BadPort,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetInfoError {
    NullError = 1,
    TooLong,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FriendAddError {
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
    FriendNotFound = 1,
    NotFound,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileSendError {
    NullError = 1,
    FriendNotFound,
    FriendNotConnected,
    NameTooLong,
    TooMany,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileSendChunkError {
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
    NullError = 1,
    FriendNotFound,
    FriendNotConnected,
    Invalid,
    Empty,
    TooLong,
    SendQ,
}