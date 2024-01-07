//
// THIS FILE IS INTENDED TO SOLUTION
// PROBLEMS WITH A COMPILER WITHOUT A
// STANDARD GROWTH LIBRARY. SOME LIBS
// FUNCTIONS WILL BE IMPORTED FROM HERE
// INTO OTHER PARTS OF THE CODE
//

use libc::{c_char, c_int, passwd, size_t, uid_t, FILE};

pub type CSTR = *const c_char;

pub fn c_str(string: &str) -> CSTR {
    return string.as_ptr() as CSTR;
}

#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {
    pub fn printf(format: CSTR, ...) -> c_int;
    pub fn fopen(filename: CSTR, mode: CSTR) -> *mut FILE;
    pub fn getline(
        lineptr: *mut *mut c_char,
        n: *mut size_t,
        stream: *mut FILE,
    );
    pub fn geteuid() -> uid_t;
    pub fn getpwuid(uid: uid_t) -> *mut passwd;
    pub fn strcat(s: *mut c_char, ct: *const c_char);
    pub fn gethostname(
        name: *mut c_char,
        len: size_t,
    ) -> c_int;
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}


