use crate::libc::*;
use crate::logos;
use crate::palette;
use crate::FetchInfo;
use crate::SystemFormat;
use core::slice;
use libc::{c_int, c_char};


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
        match get_str(args_array, i) {
            Ok(arg) => {
                arg_str = arg;
            }
            Err(_) => {
                return Err(69);
            }
        }

        let argument;
        match search_argument(arg_str) {
            Ok(arg) => {
                argument = arg;
            }
            Err(_) => {
                unsafe {
                    printf(
                        c_str("Unexpected argument: %s\n\0"),
                        arg_str,
                    );
                }

                return Err(69);
            }
        }

        if argument.short_str == "-l\0"
            || argument.short_str == "-p\0"
        {
            i += 1;
            match get_str(args_array, i) {
                Ok(arg) => {
                    arg_str = arg;
                }
                Err(_) => {
                    return Err(69);
                }
            }
        }

        let function_argument = argument.fn_argument;
        match function_argument(system, settings, arg_str) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        i += 1;
    }

    return Ok(());
}


struct Argument {
    short_str: &'static str,
    long_str: &'static str,
    fn_argument: fn(
        &mut SystemFormat,
        &mut FetchInfo,
        &'static str,
    ) -> Result<(), c_int>,
    description: &'static str,
}

impl Argument {
    const fn new(
        s_s: &'static str,
        l_s: &'static str,
        desc: &'static str,
        fn_arg: fn(
            &mut SystemFormat,
            &mut FetchInfo,
            &'static str,
        ) -> Result<(), c_int>,
    ) -> Self {
        Self {
            short_str: s_s,
            long_str: l_s,
            fn_argument: fn_arg,
            description: desc,
        }
    }
}

fn search_argument(key: &str) -> Result<&Argument, ()> {
    for i in ALL_ARGS {
        if i.short_str == key || i.long_str == key {
            return Ok(i);
        }
    }

    return Err(());
}

fn get_str(array: &[*const u8], i: usize) -> Result<&str, ()> {
    if i >= array.len() {
        unsafe {
            printf(c_str("Get argument error\n\0"));
        }
        return Err(());
    }
    let arg = array[i];

    let arg_str = unsafe {
        core::str::from_utf8_unchecked(slice::from_raw_parts(
            arg,
            strlen(arg as CSTR) + 1,
        ))
    };

    return Ok(arg_str);
}


const HELP_ARG: Argument = Argument::new(
    "-h\0",
    "--help\0",
    "Print this help screen\0",
    help,
);
const VERSION_ARG: Argument = Argument::new(
    "-v\0",
    "--version\0",
    "Print version info\0",
    version,
);
const LOGO_ARG: Argument =
    Argument::new("-l\0", "--logo\0", "Change logo\0", change_logo);
const PALETTE_ARG: Argument = Argument::new(
    "-p\0",
    "--palette\0",
    "Change palette\0",
    change_palette,
);

const ALL_ARGS: &[Argument] = &[HELP_ARG, VERSION_ARG, LOGO_ARG, PALETTE_ARG];



#[rustfmt::skip]
fn help(
    _sf: &mut SystemFormat,
    _fi: &mut FetchInfo,
    _arg: &'static str,
) -> Result<(), c_int> {
    unsafe {
        printf(c_str("LOOK AT YOUR SYSTEM WITH OTHER HAND!\n\0"));
        printf(c_str("Really gigachad fetch.\n\0"));
        let _ = version(_sf, _fi, _arg);
        printf(c_str("\nBy WagnerW man\n\0"));
        printf(c_str("Released under the MIT.\n\n\0"));
        
        for arg in ALL_ARGS {
            printf(
                c_str("%-2s %-9s %s\n\0"),
                c_str(arg.short_str),
                c_str(arg.long_str),
                c_str(arg.description),
            );
        }
    }

    return Err(0);
}

fn version(
    _sf: &mut SystemFormat,
    _fi: &mut FetchInfo,
    _arg: &'static str,
) -> Result<(), c_int> {
    let version = [0; 100];
    unsafe {
        sprintf(
            version.as_ptr() as *mut c_char,
            c_str("%s\0"),
            c_str(env!("CARGO_PKG_VERSION")),
        );
    }

    unsafe {
        printf(c_str("chadfetch %s\n\0"), c_str(&version));
    }

    return Err(0);
}

fn change_logo(
    sf: &mut SystemFormat,
    _fi: &mut FetchInfo,
    arg: &'static str,
) -> Result<(), c_int> {
    match logos::search_logo(arg) {
        Ok(logo) => {
            sf.logo = logo;
        }
        Err(_) => {
            unsafe {
                printf(c_str("Unexpected Logo: %s"), c_str(arg));
            }

            return Err(69);
        }
    }

    return Ok(());
}

fn change_palette(
    sf: &mut SystemFormat,
    _fi: &mut FetchInfo,
    arg: &'static str,
) -> Result<(), c_int> {
    match palette::search_palette(arg) {
        Ok(palette) => {
            sf.palette = palette;
        }
        Err(_) => {
            unsafe {
                printf(c_str("Unexpected Palette: %s"), c_str(arg));
            }

            return Err(69);
        }
    }

    return Ok(());
}
