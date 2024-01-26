#![allow(unused_variables, invalid_value)]

use crate::libc::{
    c_str, fgets, fopen, fread, fscanf, geteuid, gethostname,
    getpwuid, malloc, opendir, popen, readdir, sprintf,
    stat as stat_func, strchr, strcpy, strlen, strstr, strtoll,
    time, uname, CSTR,
};

use libc::{
    c_char, c_int, c_void, size_t, sscanf, stat as stat_struct,
    utsname,
};

use core::mem::MaybeUninit;
use core::slice;

use crate::all_systems::{SystemFormat, ALL_SYSTEMS, OS};

pub mod bsd;
pub mod linux;
mod linux_pkgs;
mod unix;

use crate::utils;
use crate::utils::LEN_STRING;

impl SystemFormat<'_> {
    pub fn get_system() -> Self {
        let os = unix::get_os();
        let os_name = os.get_os_name().as_ptr() as CSTR;

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

    pub fn user_host(&self) -> CSTR {
        return unix::user_host(self);
    }

    pub fn os(&self, info_space: size_t) -> CSTR {
        return unix::os(self, info_space);
    }

    pub fn device(&self, info_space: size_t) -> CSTR {
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

    pub fn kernel(&self, info_space: size_t) -> CSTR {
        return unix::kernel(self, info_space);
    }

    pub fn uptime(&self, info_space: size_t) -> CSTR {
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

    pub fn memory(&self, info_space: size_t) -> CSTR {
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

    pub fn pkgs(&self, info_space: size_t) -> CSTR {
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
}
