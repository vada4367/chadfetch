#![allow(
    unreachable_patterns,
    unused_variables,
    unused_imports,
    invalid_value
)]

use crate::libc::{
    c_str, fgets, fopen, fread, fscanf, geteuid, gethostname,
    getpwuid, glob, malloc, printf, sprintf, stat as stat_func,
    strcat, strchr, strcpy, strlen, strstr,
    sysinfo as sysinfo_function, uname, CSTR,
};

use libc::{
    c_char, c_int, c_void, glob_t, size_t, sscanf,
    stat as stat_struct, sysinfo as sysinfo_struct, utsname,
};

use crate::fetch_info::FetchInfo;

use core::mem::MaybeUninit;
use core::slice;

use crate::all_systems::{SystemFormat, ALL_SYSTEMS, OS};

mod linux;

const LEN_STRING: usize = 1;

impl SystemFormat<'_> {
    pub fn get_system() -> Self {
        let os_name = c_str(
            &Self::get_os_name()[5..Self::get_os_name().len()],
        );

        // DELETE ALL "
        let mut p = c_str(&os_name);
        loop {
            // 34 IS "
            p = unsafe { strchr(p, 34 as c_int) };
            if p == core::ptr::null() {
                break;
            }
            unsafe { strcpy(p as *mut c_char, p.add(1)) };
        }

        for system in ALL_SYSTEMS {
            if c_str(system.name) == os_name {
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
                printf(c_str("%s\n\0"), self.logo.logo);
            }

            // MOVE THE CURSOR TO
            // THE BEGGINNING OF
            // THE OUTPUT
            printf(c_str("\x1B[%dA\0"), dy + 1);

            // DY IS LEN (Y) OF LOGO
            dy -= self.print_all_info(settings);

            // MOVE THE CURSOR TO
            // THE END OF THE OUTPUT
            printf(c_str("\x1B[%dB\0"), dy + 1);
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

        if settings.logo {
            print_space = self.logo.w as i32;
        }

        // THIS VAR NEEDS TO MOVE CURSOR
        // TO THE END OF OUTPUT
        let mut count_of_info = 0;

        if settings.user_host {
            Self::print_info(self.user_host(), print_space);
            count_of_info += 1;
        }

        // MAX_LENGTH NEEDS TO MAKE A CORRECT
        // SPACES (ALL INFO ON ONE "Y" LINE)
        let max_length = settings.max_length();

        if settings.os {
            Self::print_info(self.os(max_length - 2), print_space);
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
            Self::print_info(self.pkgs(max_length - 4), print_space);
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
        match self.os {
            OS::Linux => {
                return linux::user_host(self);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn os(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::os(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn device(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::device(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn kernel(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::kernel(self, info_space);
            }
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn uptime(&self, info_space: size_t) -> CSTR {
        match self.os {
            OS::Linux => {
                return linux::uptime(self, info_space);
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
            _ => {
                return c_str("unknown_os\0");
            }
        }
    }

    fn get_os_name() -> &'static str {
        return linux::get_os_name();
    }
}