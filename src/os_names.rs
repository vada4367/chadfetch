#![allow(dead_code)]
use crate::libc::CSTR;

const void_str: CSTR = "huy\0".as_ptr() as CSTR;
