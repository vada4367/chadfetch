#![no_std]
#![no_main]

mod args;
use crate::args::*;

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

use core::slice;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    args::read_args(_argc, _argv);

    let mut system = SystemFormat::get_system();

    let settings = FetchInfo {
        logo: true,
        user_host: true,
        os: true,
        device: true,
        kernel: true,
        uptime: true,
        pkgs: true,
        memory: true,
    };

    system.logo = GIGACHAD_LOGO;
    system.palette = GIGACHAD_PALETTE;

    //system.print_fetch(settings);

    return 0isize;
}

fn help() {
    todo!();
}
