#![allow(
    unreachable_patterns,
    unused_variables,
    unused_imports
)]

use crate::libc::{
    c_str, fopen, geteuid, gethostname, getline, getpwuid,
    printf, strcat, CSTR,
};

use libc::c_char;

use crate::logos::*;
use crate::os_names::*;
use crate::fetch_info::FetchInfo;

#[derive(PartialEq)]
pub enum System {
    Void,
    Unknown,
}

impl System {
    pub fn get_system() -> System {
        let mut hostname: [c_char; 256] = [0; 256];
        let mut len = 256;

        unsafe {
            gethostname(hostname.as_mut_ptr() as *mut i8, len);
            printf(c_str("%s\n\0"), hostname.as_ptr() as CSTR);
        }

        match hostname {
            void_str => return System::Void,
            _ => return System::Unknown,
        }
    }

    pub fn print_fetch(&self, settings: FetchInfo) {
        unsafe {
            if settings.logo {
                self.logo();
            }
            if settings.user_host {
                Self::print_info(Self::user_host());
            }
        }
    }

    fn print_info(info: CSTR) {
        unsafe {
            printf(c_str("                   %s\n\0"), info);
        }
    }

    pub fn user_host() -> *const i8 {
        let user;

        unsafe {
            user = (*getpwuid(geteuid())).pw_name;
        }

        let hostname: *mut i8 = &mut 0;
        let mut len = 0;

        unsafe {
            gethostname(hostname, len);
            strcat(user, c_str("@\0"));
            strcat(user, hostname);
            strcat(user, c_str("\0"))
        }

        user
    }

    pub unsafe fn logo(&self) {
        let mut logo = "";
        match self {
            Self::Void => {
                logo = VOID_LOGO;
            }
            _ => {}
        }

        printf(c_str("%s\n\0"), c_str(&logo[1..logo.len()]));
    }
}
