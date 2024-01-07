#![no_std]
#![no_main]

mod libc;

mod system;
use crate::system::System;

mod fetch_info;
use fetch_info::FetchInfo;

mod logos;
mod os_names;

#[no_mangle]
fn main(_argc: *const *const i8, _argv: isize) -> isize {
    let system = System::get_system();
    let settings = FetchInfo {
        logo: true,
        user_host: true,
    };
    system.print_fetch(settings);

    return 0isize;
}
