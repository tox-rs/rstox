extern crate rstox;

use std::env::args;
use std::fs::File;
use std::io::{ Read, Write };
use rstox::encryptsave;


macro_rules! read {
    ( $p:expr ) => {{
        let mut data = Vec::new();
        File::open($p).unwrap()
            .read_to_end(&mut data).ok();
        data
    }}
}

fn main() {
    let mut argv = args().skip(1);

    let result = match argv.next() {
        Some(s) => match s.as_ref() {
            "encrypt" => encryptsave::pass_encrypt(
                &argv.next().unwrap().as_bytes(),
                &read!(argv.next().unwrap())
            ).unwrap(),
            "decrypt" => encryptsave::pass_decrypt(
                &argv.next().unwrap().as_bytes(),
                &read!(argv.next().unwrap())
            ).unwrap(),
            _ => panic!("encrypt | decrypt")
        },

        None => panic!("encryptfile (encrypt | decrypt) KEY FILE OUTFILE")
    };

    File::create(argv.next().unwrap()).unwrap()
        .write(&result).ok();
}
