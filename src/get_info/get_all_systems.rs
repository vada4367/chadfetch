use crate::get_info::*;

pub fn kernel(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;
    let result = [0; LEN_STRING];

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

    if name.sysname.as_ptr() as CSTR == c_str("OpenBSD\0") {
        return OS::BSD;
    }
    if name.sysname.as_ptr() as CSTR == c_str("Linux\0") {
        return OS::Linux;
    }

    return OS::Unknown;
}
