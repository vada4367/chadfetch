#![allow(
    unreachable_patterns,
    unused_variables,
    unused_imports
)]

use crate::libc::{
    c_str, fopen, fscanf, geteuid, gethostname, getpwuid,
    printf, strcat, CSTR,
};

use libc::c_char;

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
            if settings.logo {
                self.logo();
            }

            //printf(c_str("\x1B[6A\0"));

            if settings.user_host {
                Self::print_info(Self::user_host());
            }

            //printf(c_str("\x1B[11B\0"));
        }
    }

    fn print_info(info: CSTR) {
        unsafe {
            //printf(c_str("\x1B[10D"));
            printf(c_str("%s\n\0"), info);
        }
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

    unsafe fn logo(&self) {
        let mut logo = " \0";
        match self {
            Self::Void => {
                logo = VOID_LOGO;
            }
            _ => {}
        }

        printf(
            c_str("%s\n\0"),
            c_str(&logo[1..logo.len()]),
        );
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
