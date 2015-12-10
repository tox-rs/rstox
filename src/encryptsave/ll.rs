use encryptsave::errors;


pub const PASS_SALT_LENGTH: usize = 32;
pub const PASS_KEY_LENGTH: usize = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Tox_PassKey {
    salt: [u8; PASS_SALT_LENGTH],
    key: [u8; PASS_KEY_LENGTH]
}

#[link(name = "toxencryptsave")]
extern "C" {
    pub fn tox_get_salt(data: *const u8, salt: *mut u8) -> bool;
    pub fn tox_derive_key_from_pass(
        passphrase: *const u8,
        pplength: usize,
        out_key: *mut Tox_PassKey,
        error: *mut errors::KeyDerivationError
    ) -> bool;
    pub fn tox_derive_key_with_salt(
        passphrase: *const u8,
        pplength: usize,
        salt: *const u8,
        out_key: *mut Tox_PassKey,
        error: *mut errors::KeyDerivationError
    ) -> bool;
    pub fn tox_pass_key_encrypt(
        data: *const u8,
        data_len: usize,
        key: *const Tox_PassKey,
        out: *mut u8,
        error: *mut errors::EncryptionError
    ) -> bool;
    pub fn tox_pass_encrypt(
        data: *const u8,
        data_len: usize,
        passphrase: *const u8,
        pplength: usize,
        out: *mut u8,
        error: *mut errors::EncryptionError
    ) -> bool;
    pub fn tox_pass_key_decrypt(
        data: *const u8,
        length: usize,
        key: *const Tox_PassKey,
        out: *mut u8,
        error: *mut errors::DecryptionError
    ) -> bool;
    pub fn tox_pass_decrypt(
        data: *const u8,
        length: usize,
        passphrase: *const u8,
        pplength: usize,
        out: *mut u8,
        error: *mut errors::DecryptionError
    ) -> bool;
    pub fn tox_is_data_encrypted(data: *const u8) -> bool;
}
