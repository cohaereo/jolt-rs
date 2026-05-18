#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Ensure the link_cplusplus crate doesn't get optimized out
#[allow(unused_extern_crates)]
extern crate link_cplusplus;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
