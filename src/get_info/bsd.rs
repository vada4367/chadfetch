use crate::get_info::*;
extern crate libc;

pub fn device(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let file_ptr = unsafe { popen(c_str("/sbin/sysctl -n hw.product\0"), c_str("r\0")) };

    if file_ptr.is_null() {
        return c_str("popen_error\0");
    }

    let mut output = [0; LEN_STRING + 100];

    unsafe {
        fgets(output.as_ptr() as *mut c_char, (LEN_STRING + 100).try_into().unwrap(), file_ptr);
    }

    c_str(&output)
}


pub fn uptime(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    todo!();
}


pub fn memory(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    todo!();
}

pub fn pkgs(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    todo!();
}


pub fn get_os_name() -> &'static str {
    "OpenBSD"
}


