#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyDerivationError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    Failed
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EncryptionError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    KeyDerivationFailed,
    Failed
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DecryptionError {
    #[doc(hidden)] NoError = 0,
    NullError = 1,
    InvalidLength,
    BadFormat,
    KeyDerivationFailed,
    Failed
}
