use crate::libc::*;
use ::libc::{c_char, size_t};

pub const LEN_STRING: usize = 16;

pub fn spaces(info_space: size_t) -> [c_char; 20] {
    let mut spaces = [0x20 as c_char; 20];
    spaces[info_space] = 0 as c_char;

    spaces
}

pub fn time(secs: size_t) -> CSTR {
    let result = [0 as c_char; LEN_STRING + 16];

    let updays = [0 as c_char; LEN_STRING + 4];
    let uphours = [0 as c_char; LEN_STRING + 4];
    let upmins = [0 as c_char; LEN_STRING + 4];

    unsafe {
        sprintf(
            updays.as_ptr() as *mut c_char,
            c_str("%dd \0"),
            secs / 86400,
        );
        sprintf(
            uphours.as_ptr() as *mut c_char,
            c_str("%dh \0"),
            secs % 86400 / 3600,
        );
        sprintf(
            upmins.as_ptr() as *mut c_char,
            c_str("%dm \0"),
            secs % 3600 / 60,
        );

        if secs / 86400 != 0 {
            strcat(
                result.as_ptr() as *mut c_char,
                updays.as_ptr() as CSTR,
            );
        }
        if secs % 86400 / 3600 != 0 {
            strcat(
                result.as_ptr() as *mut c_char,
                uphours.as_ptr() as CSTR,
            );
        }
        strcat(
            result.as_ptr() as *mut c_char,
            upmins.as_ptr() as CSTR,
        );
    }

    c_str(&result)
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
