use crate::libc::*;
use ::libc::{c_char, size_t};

pub const LEN_STRING: usize = 1;

pub fn spaces(info_space: size_t) -> [c_char; 20] {
    let mut spaces = [0x20 as c_char; 20];
    spaces[info_space] = 0 as c_char;

    spaces
}

pub fn time(secs: size_t) -> CSTR {
    let result = [0; LEN_STRING + 16];

    unsafe {
        if secs / 86400 != 0 {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%dd %dh %dm\0"),
                secs / 86400,
                secs % 86400 / 3600,
                secs % 3600 / 60,
            );
        }
        else if secs % 86400 / 3600 != 0 {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%dh %dm\0"),
                secs / 3600,
                secs % 3600 / 60,
            );
        }
        else {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%dm\0"),
                secs / 60,
            );
        }
    }

    result.as_ptr() as CSTR
}

macro_rules! delete_char {
    ($string:expr, $char:expr) => {{
        let mut p = $string;
        loop {
            p = unsafe { strchr(p, $char) };
            if p == core::ptr::null() {
                break;
            }
            unsafe { strcpy(p as *mut c_char, p.add(1)) };
        }
    }};
}
pub(crate) use delete_char;
