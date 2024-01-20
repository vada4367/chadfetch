use crate::libc::*;
use crate::logos;
use crate::palette;
use crate::FetchInfo;
use crate::SystemFormat;
use core::slice;
use libc::c_int;

pub const ALL_ARGS: &[(
    &str,
    &str,
    fn(&mut SystemFormat, &mut FetchInfo) -> Result<(), c_int>,
)] = &[
    ("-h\0", "--help\0", help),
    ("-v\0", "--version\0", version)
];

pub fn search_argument(
    key: &str,
) -> fn(&mut SystemFormat, &mut FetchInfo) -> Result<(), c_int> {
    ALL_ARGS[
        ALL_ARGS
            .iter()
            .position(|(i, j, _)| i == &key || j == &key)
            .expect("NO LOGO")
    ].2
}
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

        let arg_func = search_argument(arg_str);

        match arg_func(system, settings) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
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

fn help(
    _sf: &mut SystemFormat,
    _fi: &mut FetchInfo,
) -> Result<(), c_int> {
    unsafe {
        printf(c_str("By WagnerW man\n\0"));
        printf(c_str("Released under the MIT.\n\n\0"));
        printf(c_str(
            "-h   --help       Print this help screen\n\0",
        ));
        printf(c_str("-v   --version    Print version info\n\0"));
        printf(c_str("-ol  --off-logo   Off print logo\n\0"));
        printf(c_str("-l   --logo       Change logo\n\0"));
        printf(c_str("-p   --palette    Change palette\n\0"));
        printf(c_str("-on  --off-name   Off user_host print\n\0"));
        printf(c_str("-oos --off-os     Off os print\n\0"));
        printf(c_str(
            "-oh  --off-host   Off host (device) print\n\0",
        ));
        printf(c_str("-ok  --off-kernel Off kernel print\n\0"));
        printf(c_str("-ou  --off-uptime Off uptime print\n\0"));
        printf(c_str("-op  --off-pkgs   Off pkgs print\n\0"));
        printf(c_str("-om  --off-memory Off memory print\n\0"));
    }

    return Err(0);
}

fn version(
    _sf: &mut SystemFormat,
    _fi: &mut FetchInfo,
) -> Result<(), c_int> {
    unsafe {
        printf(c_str("chadfetch 0.2.2\n\0"));
    }

    return Err(0);
}
