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
            let (mut _logo, mut _dx, mut dy) =
                (c_str("\0"), -4, -2);
            if settings.logo {
                (_logo, _dx, dy) = self.logo();
                printf(c_str("%s\n\0"), _logo);
            }

            printf(c_str("\x1B[%dA\0"), dy + 1);

            dy -= self.print_all_info(settings);

            printf(c_str("\x1B[%dB\0"), dy + 1);
        }
    }

    fn print_info(info: CSTR, space: i32) {
        unsafe {
            printf(c_str("\x1B[%dC\0"), space + 4);
            printf(c_str("%s\n\0"), info);
        }
    }

    fn print_all_info(&self, settings: FetchInfo) -> i32 {
        let (mut _logo, mut print_space, mut _dy_logo) =
            (c_str("\0"), -4, -2);
        let mut count_of_info = 0;

        if settings.logo {
            (_logo, print_space, _dy_logo) = self.logo();
        }

        if settings.user_host {
            Self::print_info(
                Self::user_host(),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.os {
            Self::print_info(
                Self::os(0),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.device {
            Self::print_info(
                Self::device(0),
                print_space,
            );
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

    fn os(info_space: size_t) -> CSTR {
        let result: [c_char; 100] = [0; 100];
        unsafe {
            strcat(result.as_ptr() as *mut c_char, c_str("os: \0"));
            for i in 0..info_space {
                strcat(result.as_ptr() as *mut c_char, c_str(" \0"));
            }
            let os_name = Self::get_os_name();
            strcat(result.as_ptr() as *mut c_char, c_str(&os_name[5..os_name.len()]));
        }

        return result.as_ptr() as CSTR;
    }

    fn device(info_space: size_t) -> CSTR {
        let (name, version);

        unsafe {
            name = fopen(
                c_str("/sys/devices/virtual/dmi/id/product_name\0"),
                c_str("r\0"),
            );
            version = fopen(
                c_str("/sys/devices/virtual/dmi/id/product_version\0"),
                c_str("r\0"),
            );
        }

        let result: [c_char; 200] = [0; 200];
        let name_str: [c_char; 100] = [0; 100];
        let version_str: [c_char; 100] = [0; 100];
        
        unsafe {
            strcat(result.as_ptr() as *mut c_char, c_str("host: \0"));
            for i in 0..info_space {
                strcat(result.as_ptr() as *mut c_char, c_str(" \0"));
            }
            fscanf(
                name,
                c_str("%s\n\0"),
                name_str.as_ptr() as CSTR,
            );
            strcat(result.as_ptr() as *mut c_char, name_str.as_ptr() as CSTR);
            strcat(result.as_ptr() as *mut c_char, c_str(" \0"));
            fscanf(
                version,
                c_str("%s\n\0"),
                version_str.as_ptr() as CSTR,
            );
            strcat(result.as_ptr() as *mut c_char, version_str.as_ptr() as CSTR);
        }

        return result.as_ptr() as CSTR;
    }

    fn logo(&self) -> (CSTR, i32, i32) {
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
        let os_name: [c_char; 64] = [0; 64];

        unsafe {
            fscanf(
                os_release,
                c_str("%s\n\0"),
                os_name.as_ptr() as CSTR,
            );
            let os_name_slice = slice::from_raw_parts(
                os_name.as_ptr() as *const u8,
                64,
            );

            for sym in 0..64 {
                if os_name_slice[sym] == 0 {
                    return core::str::from_utf8_unchecked(
                        &os_name_slice[0..sym],
                    );
                }
            }

            return "not_found\0";
        }
    }
}
