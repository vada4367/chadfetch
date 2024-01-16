use crate::get_info::*;

pub fn user_host(sys_format: &SystemFormat) -> CSTR {
    let user = unsafe { (*getpwuid(geteuid())).pw_name };
    let hostname = unsafe { malloc(40) } as *mut c_char;

    let result = [0; LEN_STRING + 64];

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
    let mut spaces_str = utils::spaces(info_space);

    let result = [0; LEN_STRING];
    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dos %s \x1B[0;%d %s\0"),
            sys_format.palette.text,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            c_str(sys_format.name),
        );
    }

    result.as_ptr() as CSTR
}

pub fn kernel(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces_str = utils::spaces(info_space);
    let result = [0; LEN_STRING + 100];

    let mut name =
        unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

    unsafe {
        uname(&mut name);

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dkernel %s\x1B[0;%d%s %s\0"),
            sys_format.palette.text,
            spaces_str.as_ptr() as *const c_char,
            sys_format.palette.text,
            c_str(&name.sysname),
            c_str(&name.release),
        );
    }

    c_str(&result)
}

pub fn get_os() -> OS {
    let mut name =
        unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

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
        "OpenBSD\0" => {
            return OS::BSD;
        }
        _ => {
            return OS::Unknown;
        }
    }
}
