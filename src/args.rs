use crate::libc::*;
use libc::c_int;
use core::slice;

pub fn read_args(argc: isize, argv: *const *const u8) -> c_int {
    let args_array =
        unsafe { slice::from_raw_parts(argv, argc as usize) };

    let mut arg;
    let mut arg_str;


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
                return 0;
            }
            _ => {
                unsafe {
                    printf(
                        c_str("Unexpected argument: %s\n\0"),
                        arg_str.as_ptr() as CSTR,
                    );
                }
                return 1;
            }
        }
    }

    return -1;
}

fn help() {
    unsafe {
        printf(c_str(""))
    }
}
