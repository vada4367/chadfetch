#![no_std]
#![no_main]

mod args;

mod libc;

mod logos;
mod palette;

use crate::logos::*;
use crate::palette::*;

mod all_systems;
use crate::all_systems::SystemFormat;

mod get_info;

mod fetch_info;
use fetch_info::FetchInfo;

mod utils;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let settings = match args::read_args(_argc, _argv) {
        Ok(data) => data,
        Err(error_code) => {
            return error_code as isize;
        }
    };

    let mut system = SystemFormat::get_system();

    system.logo = GIGACHAD_LOGO;
    system.palette = GIGACHAD_PALETTE;

    system.print_fetch(settings);

    return 0isize;
}
