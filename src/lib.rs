//#![allow(dead_code)]
//#![allow(unused_variables)]
#![feature(box_syntax, unsafe_destructor)]
#![feature(libc, core, collections, std_misc, io, path, os)]
extern crate libc;
extern crate "core" as rust_core;

pub mod core;
pub mod av;
