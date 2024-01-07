#![allow(
    unreachable_patterns,
    unused_variables,
    unused_imports
)]

use crate::libc::{
    c_str, fopen, fscanf, geteuid, gethostname, getpwuid,
    printf, strcat, CSTR,
};

use libc::{c_char, size_t};

use crate::fetch_info::FetchInfo;
use crate::logos::*;
use crate::os_names::*;

use core::slice;


#[derive(PartialEq)]
pub enum System {
    Void,
    Unknown,
}


impl System {
    pub fn get_system() -> System {
        match Self::get_os_name() {
            VOID_STR => return System::Void,
            _ => return System::Unknown,
        }
    }


    pub fn print_fetch(&self, settings: FetchInfo) {
        unsafe {
            let (mut _logo, mut _dx, mut dy) = (c_str("\0"), 0, 0);
            if settings.logo {
                (_logo, _dx, dy) = self.logo();
                printf(c_str("%s\n\0"), _logo);
            }

            printf(c_str("\x1B[%dA\0"), dy + 1);
            
            dy -= self.print_all_info(settings);

            printf(c_str("\x1B[%dB\0"), dy + 1);
        }
    }


    fn print_info(info: CSTR, space: size_t) {
        unsafe {
            printf(c_str("\x1B[%dC\0"), space + 4);
            printf(c_str("%s\n\0"), info);
        }
    }


    fn print_all_info(&self, settings: FetchInfo) -> i32 {
        let (mut _logo, mut print_space, mut _dy_logo) = (c_str("\0"), 0, 0);
        let mut count_of_info = 0;

        if settings.logo {
            (_logo, print_space, _dy_logo) = self.logo();
        }

        if settings.user_host {
            Self::print_info(Self::user_host(), print_space);
            count_of_info += 1;
        }

        count_of_info
    }


    fn user_host() -> *const i8 {
        let user;

        unsafe {
            user = (*getpwuid(geteuid())).pw_name;
        }

        let mut hostname: [c_char; 256] = [0; 256];
        let len = 256;

        unsafe {
            gethostname(
                hostname.as_mut_ptr() as *mut i8,
                len,
            );
            strcat(user, c_str("@\0"));
            strcat(user, hostname.as_ptr() as CSTR);
        }

        user
    }


    fn logo(&self) -> (CSTR, usize, i32) {
        let mut logo = " \0";
        let mut dx = 0;
        let mut dy = 0;

        match self {
            Self::Void => {
                logo = VOID_LOGO;
                dx = 13;
                dy = 7;
            }
            _ => {}
        }


        (c_str(&logo[1..logo.len()]), dx, dy)
    }


    fn get_os_name() -> &'static str {
        let os_release;
        unsafe {
            os_release = fopen(
                c_str("/etc/os-release\0"),
                c_str("r\0"),
            );
        }
        let os_name: [c_char; 32] = [0; 32];

        unsafe {
            fscanf(
                os_release,
                c_str("%s\n\0"),
                os_name.as_ptr() as CSTR,
            );
            let os_name_slice = slice::from_raw_parts(
                os_name.as_ptr() as *const u8,
                32,
            );

            for sym in 0..32 {
                if os_name_slice[sym] == 0 {
                    return
                        core::str::from_utf8_unchecked(
                            &os_name_slice[0..sym],
                        );
                }
            }

            return "not_found\0";
        }
    }
}
