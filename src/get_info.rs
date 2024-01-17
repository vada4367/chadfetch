#![allow(
    unreachable_patterns,
    unused_variables,
    unused_imports,
    invalid_value
)]

use crate::libc::{
    c_str, fgets, fopen, fread, fscanf, geteuid, gethostname,
    getpwuid, malloc, opendir, popen, printf, readdir, sprintf,
    stat as stat_func, strcat, strchr, strcpy, strlen, strstr,
    strtoll, time, uname, CSTR,
};

use libc::{
    c_char, c_int, c_void, size_t, sscanf, stat as stat_struct,
    utsname,
};

use crate::fetch_info::FetchInfo;

use core::mem::MaybeUninit;
use core::slice;

use crate::all_systems::{SystemFormat, ALL_SYSTEMS, OS};

mod bsd;
mod linux_pkgs;
mod linux;
mod unix;

use crate::utils;
use crate::utils::LEN_STRING;

impl SystemFormat<'_> {
    pub fn get_system() -> Self {
        let os = unix::get_os();
        let mut os_name = c_str("unknown\0");

        match os {
            OS::Linux => {
                os_name = Self::get_os_name().as_ptr() as CSTR;
            }
            _ => {}
        }

        utils::delete_char!(os_name, '"' as c_int);

        for system in ALL_SYSTEMS {
            if system.os == OS::BSD && system.os == os {
                return system;
            }
            if system.os == os
                && system.name
                    == unsafe {
                        core::str::from_utf8_unchecked(
                            slice::from_raw_parts(
                                os_name as *const u8,
                                strlen(os_name) + 1,
                            ),
                        )
                    }
            {
                return system;
            }
        }

        return ALL_SYSTEMS[0];
    }

    pub fn print_fetch(&self, settings: FetchInfo) {
        unsafe {
            let mut dy = -2;

            if settings.logo {
                dy = self.logo.h as i32;
                
                let logo_chars = slice::from_raw_parts(c_str(self.logo.logo) as *const u8, strlen(c_str(self.logo.logo)) + 1);

                let mut i: usize = 0;
                let mut checking = false;

                while logo_chars[i] != 0 {
                    if logo_chars[i] == b'$' {
                        checking = true;
                    }
                    if checking && logo_chars[i] > 50 && logo_chars[i] < 60 {
                        printf(c_str("\x1B[0;%dm\0"), logo_chars[i] as c_int - 20);
                    }
                    if !checking {
                        printf(c_str("%c\0"), logo_chars[i] as c_int);
                    }
                    if checking && logo_chars[i] == b'}' {
                        checking = false;
                    }

                    i += 1;
                }
            }

            // MOVE THE CURSOR TO
            // THE BEGGINNING OF
            // THE OUTPUT
            printf(c_str("\x1B[%dA\0"), dy);

            // DY IS LEN (Y) OF LOGO
            dy -= self.print_all_info(settings);

            // MOVE THE CURSOR TO
            // THE END OF THE OUTPUT
            printf(c_str("\x1B[%dB\0"), dy);
        }
    }

    // INFO IS THE LINES AFTER LOGO
    fn print_info(info: CSTR, space: i32) {
        unsafe {
            // MOVE CURSOR TO END OF LOGO (X)
            printf(c_str("\x1B[%dC\0"), space + 4);

            printf(c_str("%s\n\0"), info);
        }
    }

    fn print_all_info(&self, settings: FetchInfo) -> i32 {
        let mut print_space = -4;

        // PRINT_SPACE IS VARIABLE FOR 
        // MAKE PLACE DATA STRINGS AFTER
        // LOGO
        if settings.logo {
            print_space = self.logo.w as i32;
        }

        // THIS VAR NEEDS TO MOVE CURSOR
        // TO THE END OF OUTPUT
        let mut count_of_info = 0;

        if settings.user_host {
            Self::print_info(
                self.user_host(),
                print_space,
            );
            count_of_info += 1;
        }

        // MAX_LENGTH NEEDS TO MAKE A CORRECT
        // SPACES (ALL INFO ON ONE "Y" LINE)
        let max_length = settings.max_length();

        if settings.os {
            Self::print_info(
                self.os(max_length - 2),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.device {
            Self::print_info(
                self.device(max_length - 4),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.kernel {
            Self::print_info(
                self.kernel(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.uptime {
            Self::print_info(
                self.uptime(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.pkgs {
            Self::print_info(
                self.pkgs(max_length - 4),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.memory {
            Self::print_info(
                self.memory(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }

        count_of_info
    }

    fn user_host(&self) -> CSTR {
        return unix::user_host(self);
    }

    fn os(&self, info_space: size_t) -> CSTR {
        return unix::os(self, info_space);
    }

    fn device(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::device(self, info_space);
            }
            OS::BSD => {
                return bsd::device(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn kernel(&self, info_space: size_t) -> CSTR {
        return unix::kernel(self, info_space);
    }

    fn uptime(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::uptime(self, info_space);
            }
            OS::BSD => {
                return bsd::uptime(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn memory(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::memory(self, info_space);
            }
            OS::BSD => {
                return bsd::memory(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn pkgs(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::pkgs(self, info_space);
            }
            OS::BSD => {
                return bsd::pkgs(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn get_os_name() -> &'static str {
        return linux::get_os_name();
    }
}
