use libc::DT_DIR;

use crate::get_info::*;
use libc::sprintf;

pub fn user_host(sys_format: &SystemFormat) -> CSTR {
    let user = unsafe { (*getpwuid(geteuid())).pw_name };
    let hostname = unsafe { malloc(40) } as *mut c_char;

    let result = [0; LEN_STRING + 16];

    unsafe {
        gethostname(hostname, LEN_STRING + 39);
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dm%s\x1B[0;%dm@\x1B[0;%dm%s\0"),
            sys_format.palette.contrast,
            user,
            sys_format.palette.text,
            sys_format.palette.contrast,
            hostname as CSTR,
        );
    }

    c_str(&result)
}

pub fn os(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);

    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmos %s\x1B[0;%dm%s\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            c_str(sys_format.name),
        );
    }

    result.as_ptr() as CSTR
}

pub fn kernel(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);

    let mut name = unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

    unsafe {
        uname(&mut name);

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmkernel %s\x1B[0;%dm%s\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as *const c_char,
            sys_format.palette.text,
            c_str(&name.release),
        );
    }

    c_str(&result)
}

pub fn get_os() -> OS {
    let mut name = unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

    unsafe {
        uname(&mut name);
    }

    let sysname = unsafe {
        core::str::from_utf8_unchecked(slice::from_raw_parts(
            name.sysname.as_ptr() as *const u8,
            strlen(name.sysname.as_ptr() as CSTR) + 1,
        ))
    };

    match sysname {
        "Linux\0" => {
            return OS::Linux;
        }
        "OpenBSD\0" | "FreeBSD\0" | "NetBSD\0" => {
            return OS::BSD;
        }
        _ => {
            return OS::Unknown;
        }
    }
}

pub fn search_pkgs(dname: CSTR) -> size_t {
    let mut dir;
    let d = unsafe { opendir(dname) };

    let mut pkgs = 0;
    if d != core::ptr::null_mut() {
        loop {
            unsafe {
                dir = readdir(d);
                if dir == core::ptr::null_mut() {
                    break;
                }

                pkgs += 1;
            }
        }
    }

    pkgs
}

pub fn search_pkgs_deeply(dname: CSTR) -> size_t {
    let mut dir;
    let d = unsafe { opendir(dname) };

    let mut pkgs = 0;
    if d != core::ptr::null_mut() {
        loop {
            unsafe {
                dir = readdir(d);
                if dir == core::ptr::null_mut() {
                    break;
                }
                if (*dir).d_type == DT_DIR {
                    let next_dir = [0 as c_char; 256];
                    sprintf(
                        next_dir.as_ptr() as *mut c_char,
                        c_str("%s%s\0"),
                        dname as *const c_char,
                        (*dir).d_name.as_ptr() as CSTR,
                    );
                    pkgs += search_pkgs(next_dir.as_ptr() as CSTR);
                }
            }
        }
    }

    pkgs
}
