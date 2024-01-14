#![allow(dead_code)]

use crate::get_info::*;

pub fn device(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let file_ptr = unsafe {
        popen(c_str("/sbin/sysctl -n hw.product\0"), c_str("r\0"))
    };

    if file_ptr.is_null() {
        return c_str("popen_error\0");
    }
    let output = [0; LEN_STRING + 100];

    let result = [0; LEN_STRING];
    unsafe {
        fgets(
            output.as_ptr() as *mut c_char,
            (LEN_STRING + 100) as i32,
            file_ptr,
        );
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("host %s%s\0"),
            spaces_str.as_ptr() as CSTR,
            c_str(&output),
        );
    }

    // DELETE ALL \n
    let mut p = result.as_ptr() as CSTR;
    loop {
        // 0x0a IS \n
        p = unsafe { strchr(p, 0x0a as c_int) };
        if p == core::ptr::null() {
            break;
        }
        unsafe { strcpy(p as *mut c_char, p.add(1)) };
    }

    c_str(&result)
}

pub fn uptime(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let boottime = unsafe {
        popen(c_str("/sbin/sysctl -n kern.boottime\0"), c_str("r\0"))
    };

    if boottime.is_null() {
        return c_str("popen_error\0");
    }

    let (bt_output, result) =
        ([0; LEN_STRING + 100], [0; LEN_STRING + 100]);
    let (bt, uptime);

    unsafe {
        fgets(
            bt_output.as_ptr() as *mut c_char,
            (LEN_STRING + 100) as i32,
            boottime,
        );

        bt = strtoll(c_str(&bt_output), core::ptr::null_mut(), 10)
            as size_t;
        uptime = time(core::ptr::null_mut()) as size_t - bt;
    }

    let updays = [0; LEN_STRING + 64];
    let uphours = [0; LEN_STRING + 5];
    let upmins = [0; LEN_STRING + 5];

    unsafe {
        sprintf(
            updays.as_ptr() as *mut c_char,
            c_str("%dd \0"),
            uptime / 86400,
        );
        sprintf(
            uphours.as_ptr() as *mut c_char,
            c_str("%dh \0"),
            uptime % 86400 / 3600,
        );
        sprintf(
            upmins.as_ptr() as *mut c_char,
            c_str("%dm \0"),
            uptime % 3600 / 60,
        );
        strcat(result.as_ptr() as *mut c_char, c_str("uptime \0"));
        strcat(
            result.as_ptr() as *mut c_char,
            spaces_str.as_ptr() as CSTR,
        );
        if uptime / 86400 != 0 {
            strcat(result.as_ptr() as *mut c_char, c_str(&updays));
        }
        if uptime % 86400 / 3600 != 0 {
            strcat(result.as_ptr() as *mut c_char, c_str(&uphours));
        }
        strcat(result.as_ptr() as *mut c_char, c_str(&upmins));
    }

    c_str(&result)
}

pub fn memory(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let physmem = unsafe {
        popen(c_str("/sbin/sysctl -n hw.physmem\0"), c_str("r\0"))
    };
    let vmstat =
        unsafe { popen(c_str("/usr/bin/vmstat\0"), c_str("r\0")) };

    if physmem.is_null() || vmstat.is_null() {
        return c_str("popen_error\0");
    }
    let pm_output = [0; LEN_STRING + 100];
    let vmstat_output = [0; LEN_STRING + 100];
    let pm;
    let (mut _r, mut _s) = (0, 0);
    let mut avm = 0;

    let result = [0; LEN_STRING];
    unsafe {
        fgets(
            pm_output.as_ptr() as *mut c_char,
            (LEN_STRING + 100) as i32,
            physmem,
        );
        pm = strtoll(c_str(&pm_output), core::ptr::null_mut(), 10)
            as size_t
            / 1024
            / 1024;

        for _ in 0..3 {
            fgets(
                vmstat_output.as_ptr() as *mut c_char,
                (LEN_STRING + 100) as i32,
                vmstat,
            );
        }
        sscanf(
            c_str(&vmstat_output),
            c_str("%d %d  %d\0"),
            &mut _r,
            &mut _s,
            &mut avm,
        );

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("memory %s%dM / %dM\0"),
            spaces_str.as_ptr() as CSTR,
            avm,
            pm,
        );
    }

    c_str(&result)
}

pub fn pkgs(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let result = [0; LEN_STRING];

    let mut dir;
    let d = unsafe { opendir(c_str("/var/db/pkg/\0")) };

    let mut pkgs = 0;
    if d != core::ptr::null_mut() {
        loop {
            unsafe {
                dir = readdir(d);
                if dir == core::ptr::null_mut() {
                    break;
                }

                pkgs += 1;
            }
        }
    }

    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("pkgs %s%d\0"),
            spaces_str.as_ptr() as CSTR,
            pkgs,
        );
    }

    c_str(&result)
}

pub fn get_os_name() -> &'static str {
    let result = [0; LEN_STRING];

    let mut name =
        unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

    unsafe {
        uname(&mut name);

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("%s\0"),
            c_str(&name.sysname),
        );
    }

    unsafe {
        core::str::from_utf8_unchecked(slice::from_raw_parts(
            c_str(&result) as *const u8,
            strlen(c_str(&result)),
        ))
    }
}
