//#![allow(dead_code)]
//#![allow(unused_variables)]
#![feature(box_syntax, unsafe_destructor, int_uint)]
#![feature(libc, core, collections, std_misc, old_io, path, os)]
extern crate libc;
extern crate "core" as rust_core;

pub mod core;
pub mod av;
pub mod utils;
