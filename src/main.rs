#![no_std]
#![no_main]

mod libc;
use libc::CSTR;

mod logos;

mod all_systems;
use crate::all_systems::SystemFormat;

mod get_info;

mod fetch_info;
use fetch_info::{FetchInfo, Colors};

mod utils;

#[no_mangle]
fn main(_argc: *const CSTR, _argv: isize) -> isize {
    let system = SystemFormat::get_system();

    let settings = FetchInfo {
        logo: true, // YES
        user_host: true, // YES
        os: true, // YES 
        device: true, // YES
        kernel: true,
        uptime: true,
        pkgs: true, // YES
        memory: true,
        colors: Colors::new(31usize, 32usize, 33usize),
    };

    system.print_fetch(settings);

    return 0isize;
}
