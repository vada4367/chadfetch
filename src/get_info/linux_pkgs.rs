use crate::get_info::*;

pub fn xbps() -> size_t {
    let mut dir;
    let d = unsafe { opendir(c_str("/var/db/xbps/\0")) };

    if d == core::ptr::null_mut() {
        return 0;
    }

    let fname = [0; LEN_STRING];
    loop {
        unsafe {
            dir = readdir(d);
            if strstr(c_str(&(*dir).d_name), c_str("pkgdb\0"))
                != core::ptr::null_mut()
            {
                sprintf(
                    fname.as_ptr() as *mut c_char,
                    c_str("/var/db/xbps/%s\0"),
                    c_str(&(*dir).d_name),
                );
                break;
            }
        }
    }
    if c_str(&fname) == core::ptr::null_mut() {
        return 0;
    }

    let installed_string = c_str("<string>installed</string>\0");

    let mut raw_file;
    unsafe {
        let f = fopen(c_str(&fname), c_str("r\0"));

        if f == core::ptr::null_mut() {
            return 0;
        }

        let mut stat =
            MaybeUninit::<stat_struct>::uninit().assume_init();

        stat_func(c_str(&fname), &mut stat);
        raw_file = malloc(stat.st_size as usize);
        fread(raw_file, 1, stat.st_size as size_t, f);
    }

    let mut count = 0;
    unsafe {
        loop {
            raw_file = strstr(raw_file as CSTR, installed_string)
                as *mut c_void;
            if raw_file == core::ptr::null_mut() {
                break;
            }
            count += 1;
            raw_file = raw_file.add(strlen(installed_string));
        }
    }

    count
}
