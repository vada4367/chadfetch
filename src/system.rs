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
use crate::logos::*;
use crate::os_names::*;

use core::mem::MaybeUninit;
use core::slice;

#[derive(PartialEq)]
pub enum System {
    Void,
    Unknown,
}

const LEN_STRING: usize = 1;

impl System {
    pub fn get_system() -> System {
        match Self::get_os_name() {
            VOID_STR => return System::Void,
            _ => return System::Unknown,
        }
    }

    pub fn print_fetch(&self, settings: FetchInfo) {
        unsafe {
            let (mut _logo, mut _dx, mut dy) = (c_str("\0"), -4, -2);

            if settings.logo {
                (_logo, _dx, dy) = self.logo();
                printf(c_str("%s\n\0"), _logo);
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
        let (mut _logo, mut print_space, mut _dy_logo) =
            (c_str("\0"), -4, -2);

        // THIS VAR NEEDS TO MOVE CURSOR
        // TO THE END OF OUTPUT
        let mut count_of_info = 0;

        if settings.logo {
            (_logo, print_space, _dy_logo) = self.logo();
        }

        if settings.user_host {
            Self::print_info(Self::user_host(), print_space);
            count_of_info += 1;
        }

        // MAX_LENGTH NEEDS TO MAKE A CORRECT
        // SPACES (ALL INFO ON ONE "Y" LINE)
        let max_length = settings.max_length();

        if settings.os {
            Self::print_info(Self::os(max_length - 2), print_space);
            count_of_info += 1;
        }
        if settings.device {
            Self::print_info(
                Self::device(max_length - 4),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.kernel {
            Self::print_info(
                Self::kernel(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.uptime {
            Self::print_info(
                Self::uptime(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.pkgs {
            Self::print_info(
                Self::pkgs(max_length - 4),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.memory {
            Self::print_info(
                Self::memory(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }

        count_of_info
    }

    fn user_host() -> CSTR {
        let user = unsafe { (*getpwuid(geteuid())).pw_name };
        let hostname = unsafe { malloc(40) } as *mut c_char;

        let result: [c_char; LEN_STRING] = [0; LEN_STRING];

        unsafe {
            gethostname(hostname, LEN_STRING + 39);
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%s@%s\0"),
                user,
                hostname as CSTR,
            );
        }

        c_str(&result)
    }

    fn os(info_space: size_t) -> CSTR {
        let result = [0; LEN_STRING];
        unsafe {
            let spaces_str = malloc(info_space) as *mut c_char;
            core::ptr::write_bytes(spaces_str, 0x20, info_space);
            let os_name = Self::get_os_name();

            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("os %s%s\0"),
                spaces_str,
                c_str(&os_name[5..os_name.len()]),
            );
        }

        // DELETE ALL "
        let mut p = c_str(&result);

        loop {
            // 34 IS "
            p = unsafe { strchr(p, 34 as c_int) };
            if p == core::ptr::null() {
                break;
            }
            unsafe { strcpy(p as *mut c_char, p.add(1)) };
        }

        c_str(&result)
    }

    fn device(info_space: size_t) -> CSTR {
        let (name, version);

        unsafe {
            name = fopen(
                c_str("/sys/devices/virtual/dmi/id/product_name\0"),
                c_str("r\0"),
            );
            version = fopen(
                c_str(
                    "/sys/devices/virtual/dmi/id/product_version\0",
                ),
                c_str("r\0"),
            );
        }

        let result = [0; LEN_STRING];
        let name_str = [0; LEN_STRING];
        let version_str = [0; LEN_STRING];

        unsafe {
            let spaces_str = malloc(info_space) as *mut c_char;
            core::ptr::write_bytes(spaces_str, 0x20, info_space);
            fscanf(name, c_str("%s\n\0"), c_str(&name_str));
            fscanf(version, c_str("%s\n\0"), c_str(&version_str));
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("host %s%s %s\0"),
                spaces_str,
                c_str(&name_str),
                c_str(&version_str),
            );
        }

        c_str(&result)
    }

    fn kernel(info_space: size_t) -> CSTR {
        let mut name = unsafe {
            MaybeUninit::<utsname>::uninit().assume_init()
        };
        let result = [0; LEN_STRING];

        unsafe {
            uname(&mut name);
            let spaces_str = malloc(info_space) as *mut c_char;
            core::ptr::write_bytes(spaces_str, 0x20, info_space);
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("kernel %s%s\0"),
                spaces_str,
                c_str(&name.release),
            );
        }

        c_str(&result)
    }

    fn uptime(info_space: size_t) -> CSTR {
        let mut sysinfo = unsafe {
            MaybeUninit::<sysinfo_struct>::uninit().assume_init()
        };

        unsafe {
            sysinfo_function(&mut sysinfo);
        }

        let uptime = sysinfo.uptime;

        let updays = [0; LEN_STRING + 64];
        let uphours = [0; LEN_STRING + 5];
        let upmins = [0; LEN_STRING + 5];

        let result = [0; LEN_STRING + 100];

        unsafe {
            sprintf(
                updays.as_ptr() as *mut c_char,
                c_str("%dd \0"),
                uptime / 86400,
            );
            sprintf(
                uphours.as_ptr() as *mut c_char,
                c_str("%dh \0"),
                uptime % 86400 / 3600,
            );
            sprintf(
                upmins.as_ptr() as *mut c_char,
                c_str("%dm \0"),
                uptime % 3600 / 60,
            );
            strcat(
                result.as_ptr() as *mut c_char,
                c_str("uptime \0"),
            );
            let spaces_str = malloc(info_space) as *mut c_char;
            core::ptr::write_bytes(spaces_str, 0x20, info_space);
            strcat(result.as_ptr() as *mut c_char, spaces_str);
            if uptime / 86400 != 0 {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(&updays),
                );
            }
            if uptime % 86400 / 3600 != 0 {
                strcat(
                    result.as_ptr() as *mut c_char,
                    c_str(&uphours),
                );
            }
            strcat(result.as_ptr() as *mut c_char, c_str(&upmins));
        }

        c_str(&result)
    }

    fn memory(info_space: size_t) -> CSTR {
        let file =
            unsafe { fopen(c_str("/proc/meminfo\0"), c_str("r\0")) };

        let mut line = [0; LEN_STRING + 30];

        let mem_available;
        let mut mem_total = 0;
        let mut sh_mem = 1;
        let mut mem_free = 1;
        let mut buffers = 1;
        let mut cached = 1;
        let mut s_reclaimable = 1;

        while mem_total == 0
            && mem_free == 0
            && buffers == 0
            && cached == 0
            && s_reclaimable == 0
            && sh_mem == 0
        {
            unsafe {
                let fgets_line = fgets(
                    line.as_mut_ptr(),
                    line.len() as c_int,
                    file,
                );

                /*
                let line_str = core::str::from_utf8_unchecked(
                    slice::from_raw_parts(
                        c_str(&line) as *const u8,
                        strlen(c_str(&line)),
                    ),
                );
                */
                if strstr(c_str(&line), c_str("MemTotal\0")) != core::ptr::null_mut() {
                    sscanf(
                        line.as_ptr() as CSTR,
                        c_str("MemTotal: %d\0"),
                        &mut mem_total,
                    );
                }
                /*
                if line_str.find("MemFree").is_some() {
                    sscanf(
                        line.as_ptr() as CSTR,
                        c_str("MemFree: %d\0"),
                        &mut mem_free,
                    );
                }
                if line_str.find("Buffers").is_some() {
                    sscanf(
                        line.as_ptr() as CSTR,
                        c_str("Buffers: %d\0"),
                        &mut buffers,
                    );
                }
                if line_str.find("Cached").is_some() {
                    sscanf(
                        line.as_ptr() as CSTR,
                        c_str("Cached: %d\0"),
                        &mut cached,
                    );
                }
                if line_str.find("SReclaimable").is_some() {
                    sscanf(
                        line.as_ptr() as CSTR,
                        c_str("SReclaimable: %d\0"),
                        &mut s_reclaimable,
                    );
                }
                if line_str.find("Shmem").is_some() {
                    sscanf(
                        line.as_ptr() as CSTR,
                        c_str("Shmem: %d\0"),
                        &mut sh_mem,
                    );
                }
                */
            }
        }

        mem_available =
            mem_free + buffers + cached + s_reclaimable - sh_mem;

        let result = [0; LEN_STRING];
        let spaces_str;

        unsafe {
            spaces_str = malloc(info_space) as *mut c_char;
            core::ptr::write_bytes(spaces_str, 0x20, info_space);
            strcat(result.as_ptr() as *mut c_char, spaces_str);

            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("memory %s%dM / %dM\0"),
                spaces_str,
                (mem_total - mem_available) / 1024,
                mem_total / 1024,
            );
        }

        c_str(&result)
    }

    fn xbps() -> size_t {
        let mut glob_var = unsafe {
            MaybeUninit::<libc::glob_t>::uninit().assume_init()
        };
        let fname;
        unsafe {
            glob(
                c_str("/var/db/xbps/pkgdb-*\0"),
                0,
                None,
                &mut glob_var,
            );
        }
        let paths = unsafe {
            slice::from_raw_parts(
                glob_var.gl_pathv,
                glob_var.gl_pathc as size_t,
            )
        };
        if paths.len() != 1 {
            return 0;
        }
        fname = paths[0] as CSTR;

        let installed_string = c_str("<string>installed</string>\0");

        let mut raw_file;
        unsafe {
            let f = fopen(fname, c_str("r\0"));

            let mut stat =
                MaybeUninit::<stat_struct>::uninit().assume_init();

            stat_func(fname, &mut stat);
            raw_file = malloc(stat.st_size as usize);
            fread(raw_file, 1, stat.st_size as size_t, f);
        }

        let mut count = 0;
        unsafe {
            loop {
                raw_file = strstr(raw_file as CSTR, installed_string)
                    as *mut c_void;
                if raw_file == core::ptr::null_mut() {
                    break;
                }
                count += 1;
                raw_file = raw_file.add(strlen(installed_string));
            }
        }

        count
    }

    fn pkgs(info_space: size_t) -> CSTR {
        let mut distro_pkgs = 0;

        let xbps_pkgs = Self::xbps();

        if xbps_pkgs != 0 {
            distro_pkgs = xbps_pkgs;
        }

        let result: [c_char; LEN_STRING] = [0; LEN_STRING];
        unsafe {
            let spaces_str = malloc(info_space) as *mut c_char;
            core::ptr::write_bytes(spaces_str, 0x20, info_space);
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("pkgs %s%d \0"),
                spaces_str,
                distro_pkgs,
            );

            // TODO strcat all pkgs for all distro's like flatpak
        }

        c_str(&result)
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
        let os_release = unsafe {
            fopen(c_str("/etc/os-release\0"), c_str("r\0"))
        };
        let os_name = [0; LEN_STRING + 40];

        unsafe {
            fscanf(os_release, c_str("%s\n\0"), c_str(&os_name));
            let os_name_slice = slice::from_raw_parts(
                os_name.as_ptr() as *const u8,
                LEN_STRING + 40,
            );

            for sym in 0..LEN_STRING + 40 {
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
