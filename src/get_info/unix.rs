use crate::get_info::*;

pub fn user_host(sys_format: &SystemFormat) -> CSTR {
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

pub fn os(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let result = [0; LEN_STRING + 16];
    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("os %s%s\0"),
            spaces_str.as_ptr() as CSTR,
            c_str(sys_format.name),
        );
    }

    c_str(&result)
}

pub fn kernel(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;
    let result = [0; LEN_STRING + 16];

    let mut name =
        unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

    unsafe {
        uname(&mut name);

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("kernel %s%s %s\0"),
            spaces_str.as_ptr() as *const c_char,
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
