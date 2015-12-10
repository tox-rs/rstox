extern crate rstox;

use std::env::args;
use std::fs::File;
use std::io::{ Read, Write };
use rstox::encryptsave;
use rstox::encryptsave::errors;

fn encrypt(key: &str, data: Vec<u8>) -> Result<Vec<u8>, errors::EncryptionError> {
    println!("file is encrypted? {}", encryptsave::is_encrypted(&data));
    encryptsave::pass_encrypt(key.as_bytes(), &data)
}

fn decrypt(key: &str, data: Vec<u8>) -> Result<Vec<u8>, errors::DecryptionError> {
    println!("file is encrypted? {}", encryptsave::is_encrypted(&data));
    encryptsave::pass_decrypt(key.as_bytes(), &data)
}

macro_rules! try_panic {
    ( $e:expr ) => {
        $e.unwrap_or_else(|err| panic!(err))
    }
}

macro_rules! read {
    ( $p:expr ) => {{
        let mut data = Vec::new();
        try_panic!(File::open($p))
            .read_to_end(&mut data).ok();
        data
    }}
}

fn main() {
    let mut argv = args().skip(1);

    let result = match argv.next() {
        Some(s) => match s.as_ref() {
            "encrypt" => try_panic!(encrypt(
                &argv.next().unwrap(),
                read!(argv.next().unwrap())
            )),
            "decrypt" => try_panic!(decrypt(
                &argv.next().unwrap(),
                read!(argv.next().unwrap())
            )),
            _ => panic!("encrypt | decrypt")
        },
        None => panic!("encryptfile (encrypt | decrypt) KEY FILE OUTFILE")
    };

    try_panic!(File::create(argv.next().unwrap()))
        .write(&result).ok();
}
