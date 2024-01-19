#![no_std]
#![no_main]

mod libc;
use libc::CSTR;

mod logos;
mod palette;

mod all_systems;
use crate::all_systems::SystemFormat;

mod get_info;

mod fetch_info;
use fetch_info::FetchInfo;

mod utils;



#[no_mangle]
fn main(_argc: *const CSTR, _argv: isize) -> isize {
    let system = SystemFormat::get_system();

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

    system.print_fetch(settings);

    return 0isize;
}
