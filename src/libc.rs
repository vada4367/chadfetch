#![allow(deprecated)]

//
// THIS FILE IS INTENDED TO SOLUTION
// PROBLEMS WITH A COMPILER WITHOUT A
// STANDARD GROWTH LIBRARY. SOME LIBS
// FUNCTIONS WILL BE IMPORTED FROM HERE
// INTO OTHER PARTS OF THE CODE
//

use libc::{
    c_char, c_int, c_longlong, c_void, dirent, passwd, size_t,
    stat as stat_struct, time_t, uid_t, utsname, DIR, FILE,
};

pub type CSTR = *const c_char;

pub fn c_str<T: ?Sized>(string: &T) -> CSTR {
    return string as *const T as CSTR;
}

#[link(name = "c")]
extern "C" {
    pub fn printf(format: CSTR, ...) -> c_int;
    pub fn fopen(filename: CSTR, mode: CSTR) -> *mut FILE;
    pub fn fscanf(stream: *mut FILE, format: CSTR, ...) -> c_int;
    pub fn geteuid() -> uid_t;
    pub fn getpwuid(uid: uid_t) -> *mut passwd;
    pub fn gethostname(name: *mut c_char, len: size_t) -> c_int;
    pub fn uname(buf: *mut utsname) -> c_int;
    pub fn sprintf(
        s: *mut c_char,
        format: *const c_char,
        ...
    ) -> c_int;
    pub fn strcpy(dst: *mut c_char, src: CSTR) -> *mut c_char;
    pub fn strchr(cs: CSTR, c: c_int) -> *mut c_char;
    pub fn strlen(cs: CSTR) -> size_t;
    pub fn strstr(cs: CSTR, ct: CSTR) -> *mut c_char;
    pub fn fread(
        ptr: *mut c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn stat(path: CSTR, buf: *mut stat_struct) -> c_int;
    pub fn fgets(
        buf: *mut c_char,
        n: c_int,
        stream: *mut FILE,
    ) -> *mut c_char;
    pub fn opendir(dirname: *const c_char) -> *mut DIR;
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
    pub fn popen(command: CSTR, mode: CSTR) -> *mut FILE;
    pub fn strtoll(
        s: CSTR,
        endp: *mut *mut c_char,
        base: c_int,
    ) -> c_longlong;
    pub fn time(time: *mut time_t) -> time_t;
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
