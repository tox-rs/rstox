mod ll;
pub mod errors;


macro_rules! tox_res {
    ( $res:ident <- $rexpr:expr, $err:ident, $r:expr ) => {
        unsafe {
            let mut $res = $rexpr;
            let mut $err = ::std::mem::uninitialized();
            if $r {
                Ok($res)
            } else {
                Err($err)
            }
        }
    };
    ( $res:ident, $err:ident, $r:expr ) => {
        tox_res!(
            $res <- ::std::mem::uninitialized(),
            $err,
            $r
        )
    }
}

pub const PASS_ENCRYPTION_EXTRA_LENGTH: usize = 80;

pub fn is_encrypted(data: &[u8]) -> bool {
    unsafe { ll::tox_is_data_encrypted(data.as_ptr()) }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ToxPassKey {
    passkey: *mut ll::Tox_PassKey
}

#[allow(unused_mut)]
impl ToxPassKey {
    fn new(passphrase: &[u8]) -> Result<ToxPassKey, errors::KeyDerivationError>  {
        let passkey = try!(tox_res!(
            passkey,
            err,
            ll::tox_derive_key_from_pass(
                passphrase.as_ptr(),
                passphrase.len(),
                passkey,
                &mut err
            )
        ));

        Ok(ToxPassKey { passkey: passkey })
    }

    fn from(passphrase: &[u8], data: &[u8]) -> Result<ToxPassKey, errors::KeyDerivationError> {
        ToxPassKey::with(passphrase, unsafe {
            let mut salt = Vec::new();
            ll::tox_get_salt(data.as_ptr(), salt.as_mut_ptr());
            salt
        })
    }

    fn with(passphrase: &[u8], salt: Vec<u8>) -> Result<ToxPassKey, errors::KeyDerivationError> {
        let passkey = try!(tox_res!(
            passkey,
            err,
            ll::tox_derive_key_with_salt(
                passphrase.as_ptr(),
                passphrase.len(),
                salt.as_ptr(),
                passkey,
                &mut err
            )
        ));

        Ok(ToxPassKey { passkey: passkey })
    }

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, errors::EncryptionError> {
        tox_res!(
            out <- {
                let len = data.len() + PASS_ENCRYPTION_EXTRA_LENGTH;
                let mut out = Vec::with_capacity(len);
                out.set_len(len);
                out
            },
            err,
            ll::tox_pass_key_encrypt(
                data.as_ptr(),
                data.len(),
                self.passkey,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, errors::DecryptionError> {
        tox_res!(
            out <- {
                let len = data.len() - PASS_ENCRYPTION_EXTRA_LENGTH;
                let mut out = Vec::with_capacity(len);
                out.set_len(len);
                out
            },
            err,
            ll::tox_pass_key_decrypt(
                data.as_ptr(),
                data.len(),
                self.passkey,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }
}

pub fn pass_encrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, errors::EncryptionError> {
    tox_res!(
        out <- {
            let len = data.len() + PASS_ENCRYPTION_EXTRA_LENGTH;
            let mut out = Vec::with_capacity(len);
            out.set_len(len);
            out
        },
        err,
        ll::tox_pass_encrypt(
            data.as_ptr(),
            data.len(),
            passphrase.as_ptr(),
            passphrase.len(),
            out.as_mut_ptr(),
            &mut err
        )
    )
}

pub fn pass_decrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, errors::DecryptionError> {
    tox_res!(
        out <- {
            let len = data.len() - PASS_ENCRYPTION_EXTRA_LENGTH;
            let mut out = Vec::with_capacity(len);
            out.set_len(len);
            out
        },
        err,
        ll::tox_pass_decrypt(
            data.as_ptr(),
            data.len(),
            passphrase.as_ptr(),
            passphrase.len(),
            out.as_mut_ptr(),
            &mut err
        )
    )
}
