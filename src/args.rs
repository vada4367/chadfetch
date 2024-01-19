use crate::libc::*;
use crate::logos;
use crate::palette;
use crate::FetchInfo;
use crate::SystemFormat;
use core::slice;
use libc::c_int;

pub fn read_args(
    argc: isize,
    argv: *const *const u8,
    system: &mut SystemFormat,
    settings: &mut FetchInfo,
) -> Result<(), c_int> {
    let args_array =
        unsafe { slice::from_raw_parts(argv, argc as usize) };

    let mut arg_str;

    let mut i = 1;
    while i < args_array.len() {
        arg_str = get_str(args_array, i);

        match arg_str {
            "-h\0" | "--help\0" => {
                version();
                help();
                return Err(0);
            }
            "-v\0" | "--version\0" => {
                version();
                return Err(0);
            }
            "-ol\0" | "--off-logo\0" => {
                settings.logo = false;
            }
            "-l\0" | "--logo\0" => {
                i += 1;

                arg_str = get_str(args_array, i);

                system.logo = logos::search_logo(arg_str);
            }
            "-p\0" | "--palette\0" => {
                i += 1;

                arg_str = get_str(args_array, i);

                system.palette = palette::search_palette(arg_str);
            }
            "-on\0" | "--off-name\0" => {
                settings.user_host = false;
            }
            "-oos\0" | "--off-os\0" => {
                settings.os = false;
            }
            "-oh\0" | "--off-host\0" => {
                settings.device = false;
            }
            "-ok\0" | "--off-kernel\0" => {
                settings.kernel = false;
            }
            "-ou\0" | "--off-uptime\0" => {
                settings.uptime = false;
            }
            "-op\0" | "--off-pkgs\0" => {
                settings.pkgs = false;
            }
            "-om\0" | "--off-memory\0" => {
                settings.memory = false;
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

        i += 1;
    }

    return Ok(());
}

fn get_str(array: &[*const u8], i: usize) -> &str {
    let arg = array[i];

    let arg_str = unsafe {
        core::str::from_utf8_unchecked(slice::from_raw_parts(
            arg,
            strlen(arg as CSTR) + 1,
        ))
    };

    return arg_str;
}

fn help() {
    unsafe {
        printf(c_str("By WagnerW man\n\0"));
        printf(c_str("Released under the MIT.\n\n\0"));
        printf(c_str("-h   --help       Print this help screen\n\0"));
        printf(c_str("-v   --version    Print version info\n\0"));
        printf(c_str("-ol  --off-logo   Off print logo\n\0"));
        printf(c_str("-l   --logo       Change logo\n\0"));
        printf(c_str("-p   --palette    Change palette\n\0"));
        printf(c_str("-on  --off-name   Off user_host print\n\0"));
        printf(c_str("-oos --off-os     Off os print\n\0"));
        printf(c_str("-oh  --off-host   Off host (device) print\n\0"));
        printf(c_str("-ok  --off-kernel Off kernel print\n\0"));
        printf(c_str("-ou  --off-uptime Off uptime print\n\0"));
        printf(c_str("-op  --off-pkgs   Off pkgs print\n\0"));
        printf(c_str("-om  --off-memory Off memory print\n\0"));
    }
}

fn version() {
    unsafe {
        printf(c_str("chadfetch 0.2.2\n\0"));
    }
}
