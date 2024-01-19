use crate::libc::*;
use crate::FetchInfo;
use core::slice;
use libc::c_int;

pub fn read_args(
    argc: isize,
    argv: *const *const u8,
) -> Result<FetchInfo, c_int> {
    let mut settings = FetchInfo {
        logo: true,
        user_host: true,
        os: true,
        device: true,
        kernel: true,
        uptime: true,
        pkgs: true,
        memory: true,
    };

    let args_array =
        unsafe { slice::from_raw_parts(argv, argc as usize) };

    let arg;
    let arg_str;

    for i in 1..args_array.len() {
        arg = args_array[i];

        arg_str = unsafe {
            core::str::from_utf8_unchecked(slice::from_raw_parts(
                arg,
                strlen(arg as CSTR) + 1,
            ))
        };

        match arg_str {
            "-h\0" | "--help\0" => {
                help();
                return Err(0);
            }
            "-v\0" | "--version\0" => {
                version();
                return Err(0);
            }
            _ => {
                unsafe {
                    printf(
                        c_str("Unexpected argument: %s\n\0"),
                        arg_str.as_ptr() as CSTR,
                    );
                }
                return Err(-1);
            }
        }
    }

    return Ok(settings);
}

fn help() {
    unsafe {
        printf(c_str("By WagnerW man\n\0"));
        printf(c_str("Released under the MIT.\n\n\0"));
        printf(c_str("-h --help    Print this help screen\n\0"));
        printf(c_str("-v --version Print version info\n\0"));
    }
}

fn version() {
    unsafe {
        printf(c_str("chadfetch 0.2.2\n\0"));
    }
}
