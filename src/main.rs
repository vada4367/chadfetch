#![no_std]
#![no_main]

mod args;

mod libc;

mod logos;
mod palette;

mod all_systems;
use crate::all_systems::SystemFormat;

mod get_info;

mod fetch_info;
use fetch_info::FetchInfo;

mod utils;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut settings = FetchInfo::get_full_info();
    let mut system = SystemFormat::get_system();

    match args::read_args(_argc, _argv, &mut system, &mut settings) {
        Ok(_) => {}
        Err(error_code) => {
            return error_code as isize;
        }
    };

    system.print_fetch(settings);

    return 0isize;
}
