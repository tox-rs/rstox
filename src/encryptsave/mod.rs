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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ToxPassKey {
    passkey: ll::Tox_PassKey
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
                err
            )
        ));

        Ok(ToxPassKey { passkey: passkey })
    }

    fn with(passphrase: &[u8], salt: &[u8]) -> Result<ToxPassKey, errors::KeyDerivationError> {
        let passkey = try!(tox_res!(
            passkey,
            err,
            ll::tox_derive_key_with_salt(
                passphrase.as_ptr(),
                passphrase.len(),
                salt.as_ptr(),
                passkey,
                err
            )
        ));

        Ok(ToxPassKey { passkey: passkey })
    }

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, errors::EncryptionError> {
        tox_res!(
            out <- Vec::new(),
            err,
            ll::tox_pass_key_encrypt(
                data.as_ptr(),
                data.len(),
                &self.passkey,
                out.as_mut_ptr(),
                err
            )
        )
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, errors::DecryptionError> {
        tox_res!(
            out <- Vec::new(),
            err,
            ll::tox_pass_key_decrypt(
                data.as_ptr(),
                data.len(),
                &self.passkey,
                out.as_mut_ptr(),
                err
            )
        )
    }
}

pub fn pass_encrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, errors::EncryptionError> {
    unimplemented!()
}

pub fn pass_decrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, errors::DecryptionError> {
    unimplemented!()
}
