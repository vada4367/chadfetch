use crate::libc::*;
use ::libc::{size_t, c_char};


pub const LEN_STRING: usize = 1;

pub fn spaces(info_space: size_t) -> [c_char; 20] {
    let mut spaces = [0x20 as c_char; 20];
    spaces[info_space] = 0 as c_char;

    spaces
}

pub fn time(secs: size_t) -> CSTR {
    let result = [0; LEN_STRING + 64];

    let updays = [0; LEN_STRING + 32];
    let uphours = [0; LEN_STRING + 4];
    let upmins = [0; LEN_STRING + 4];

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
            strcat(result.as_ptr() as *mut c_char, c_str(&updays));
        }
        if secs % 86400 / 3600 != 0 {
            strcat(result.as_ptr() as *mut c_char, c_str(&uphours));
        }
        strcat(result.as_ptr() as *mut c_char, c_str(&upmins));
    }
    c_str(&result)
}
