#![allow(
    unreachable_patterns,
    unused_variables,
    unused_imports,
    invalid_value
)]

use crate::libc::{
    c_str, fopen, fread, fscanf, geteuid, gethostname,
    getpwuid, glob, malloc, printf, sprintf,
    stat as stat_func, strcat, strchr, strcpy, strlen,
    strstr, sysinfo as sysinfo_function, uname, CSTR,
};

use libc::{
    c_char, c_int, c_void, glob_t, size_t,
    stat as stat_struct, sysinfo as sysinfo_struct,
    utsname,
};

use crate::fetch_info::FetchInfo;
use crate::logos::*;
use crate::os_names::*;

use core::mem::MaybeUninit;
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
            Self::print_info(Self::os(4), print_space);
            count_of_info += 1;
        }
        if settings.device {
            Self::print_info(Self::device(2), print_space);
            count_of_info += 1;
        }
        if settings.kernel {
            Self::print_info(Self::kernel(0), print_space);
            count_of_info += 1;
        }
        if settings.uptime {
            Self::print_info(Self::uptime(0), print_space);
            count_of_info += 1;
        }
        if settings.pkgs {
            Self::print_info(Self::pkgs(1), print_space);
            count_of_info += 1;
        }
        if settings.memory {
            Self::print_info(Self::memory(0), print_space);
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
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("os \0"),
            );
            for i in 0..info_space {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(" \0"),
                );
            }
            let os_name = Self::get_os_name();
            strcat(
                result.as_ptr() as *mut c_char,
                c_str(&os_name[5..os_name.len()]),
            );
        }

        let mut p: CSTR = result.as_ptr() as CSTR;

        loop {
            unsafe {
                p = strchr(p, 34 as c_int);
                if p == core::ptr::null() {
                    break;
                }
                strcpy(p as *mut c_char, p.add(1));
            }
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
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("host \0"),
            );
            for i in 0..info_space {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(" \0"),
                );
            }
            fscanf(
                name,
                c_str("%s\n\0"),
                name_str.as_ptr() as CSTR,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                name_str.as_ptr() as CSTR,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                c_str(" \0"),
            );
            fscanf(
                version,
                c_str("%s\n\0"),
                version_str.as_ptr() as CSTR,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                version_str.as_ptr() as CSTR,
            );
        }

        return result.as_ptr() as CSTR;
    }

    fn kernel(info_space: size_t) -> CSTR {
        let mut name = unsafe {
            MaybeUninit::<utsname>::uninit().assume_init()
        };
        let result: [c_char; 100] = [0; 100];

        unsafe {
            uname(&mut name);
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("kernel \0"),
            );
            for i in 0..info_space {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(" \0"),
                );
            }
            strcat(
                result.as_ptr() as *mut c_char,
                name.release.as_ptr() as CSTR,
            );
        }

        return result.as_ptr() as CSTR;
    }

    fn uptime(info_space: size_t) -> CSTR {
        let mut sysinfo = unsafe {
            MaybeUninit::<sysinfo_struct>::uninit()
                .assume_init()
        };

        unsafe {
            sysinfo_function(&mut sysinfo);
        }

        let uptime: u64 = sysinfo.uptime;

        let updays: [c_char; 50] = [0; 50];
        let uphours: [c_char; 50] = [0; 50];
        let upmins: [c_char; 50] = [0; 50];

        let result: [c_char; 150] = [0; 150];

        unsafe {
            sprintf(
                updays.as_ptr() as *mut c_char,
                c_str("%d\0"),
                uptime / 86400,
            );
            sprintf(
                uphours.as_ptr() as *mut c_char,
                c_str("%d\0"),
                uptime % 86400 / 3600,
            );
            sprintf(
                upmins.as_ptr() as *mut c_char,
                c_str("%d\0"),
                uptime % 3600 / 60,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("uptime \0"),
            );
            for i in 0..info_space {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(" \0"),
                );
            }
            if uptime / 86400 != 0 {
                strcat(
                    result.as_ptr() as *mut c_char,
                    updays.as_ptr() as CSTR,
                );
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str("d \0"),
                );
            }
            if uptime % 86400 / 3600 != 0 {
                strcat(
                    result.as_ptr() as *mut c_char,
                    uphours.as_ptr() as CSTR,
                );
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str("h \0"),
                );
            }
            strcat(
                result.as_ptr() as *mut c_char,
                upmins.as_ptr() as CSTR,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("m\0"),
            );
        }

        result.as_ptr() as CSTR
    }

    fn memory(info_space: size_t) -> CSTR {
        let mut sysinfo = unsafe {
            MaybeUninit::<sysinfo_struct>::uninit()
                .assume_init()
        };

        unsafe {
            sysinfo_function(&mut sysinfo);
        }

        let memory: [c_char; 100] = [0; 100];
        unsafe {
            sprintf(
                memory.as_ptr() as *mut c_char,
                c_str("%dM / %dM\0"),
                (sysinfo.totalram - sysinfo.freeram)
                    / 1024
                    / 1024,
                sysinfo.totalram / 1024 / 1024,
            );
        }

        let result: [c_char; 100] = [0; 100];

        unsafe {
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("memory \0"),
            );

            for i in 0..info_space {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(" \0"),
                );
            }
            strcat(
                result.as_ptr() as *mut c_char,
                memory.as_ptr() as CSTR,
            );
        }

        result.as_ptr() as CSTR
    }

    fn pkgs(info_space: size_t) -> CSTR {
        let mut glob_var = unsafe {
            MaybeUninit::<libc::glob_t>::uninit()
                .assume_init() };
        let fname;
        unsafe {
            glob(
                c_str("/var/db/xbps/pkgdb-*\0"),
                0,
                None,
                &mut glob_var,
            );
        }
        let paths = unsafe { slice::from_raw_parts(
            glob_var.gl_pathv,
            glob_var.gl_pathc as size_t,
        ) };
        if paths.len() == 1 {
            fname = paths[0] as CSTR;
        } else {
            return c_str("\0");
        }

        let installed_string =
            c_str("<string>installed</string>\0");
        let f = unsafe { fopen(fname, c_str("rb\0")) };
        let mut stat = unsafe { MaybeUninit::<stat_struct>::uninit()
            .assume_init() };
        unsafe {
            stat_func(fname, &mut stat);
        }

        let mut raw_file = unsafe { malloc(stat.st_size as usize) };
        unsafe {
            fread(raw_file, 1, stat.st_size as size_t, f);
        }

        let mut count = 0;
        loop {
            raw_file = unsafe {
                strstr(raw_file as CSTR, installed_string)
                    as *mut c_void };
            if raw_file == core::ptr::null_mut() {
                break;
            }
            count += 1;
            raw_file = unsafe {
                raw_file.add(strlen(installed_string)) };
        }

        let result: [c_char; 100] = [0; 100];
        unsafe {
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("pkgs: \0"),
            );
            for i in 0..info_space {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(" \0"),
                );
            }
            let pkgs: [c_char; 100] = [0; 100];
            sprintf(
                pkgs.as_ptr() as *mut c_char,
                c_str("%d\0"),
                count,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                pkgs.as_ptr() as CSTR,
            );
        }

        result.as_ptr() as CSTR
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
