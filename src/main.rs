//#![no_std]
#![no_main]

mod libc;
use libc::CSTR;

mod system;
use crate::system::System;

mod fetch_info;
use fetch_info::FetchInfo;

mod logos;
mod os_names;

#[no_mangle]
fn main(_argc: *const CSTR, _argv: isize) -> isize {
    let system = System::get_system();
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
