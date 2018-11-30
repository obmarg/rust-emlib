#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_uint = u32;
pub type c_int = i32;

// TODO: Check this is correct
pub type c_void = core::ffi::c_void;
