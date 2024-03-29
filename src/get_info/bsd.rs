use crate::get_info::*;

pub fn device(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);

    let hw_product = unsafe {
        popen(c_str("/sbin/sysctl -n hw.product\0"), c_str("r\0"))
    };

    if hw_product.is_null() {
        return c_str("popen_error\0");
    }
    let output = [0; LEN_STRING + 100];

    unsafe {
        fgets(
            output.as_ptr() as *mut c_char,
            (LEN_STRING + 100) as i32,
            hw_product,
        );
    }

    utils::delete_char!(output.as_ptr() as CSTR, '\n' as c_int);

    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmhost %s\x1B[0;%dm%s\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            c_str(&output),
        );
    }

    c_str(&result)
}

pub fn uptime(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);

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

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmuptime %s\x1B[0;%dm%s\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            utils::time(uptime),
        );
    }

    c_str(&result)
}

pub fn memory(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);

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
    let (mut _r, mut _s) = (0, 0);
    let mut avm = 0;
    let pm;

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

        // SKIP LINES (just start vmstat to understand)
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
            c_str("\x1B[0;%dmmemory %s\x1B[0;%dm%dM / %dM\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            avm,
            pm,
        );
    }

    c_str(&result)
}

pub fn pkgs(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);

    let pkgs = unix::search_pkgs(c_str("/var/db/pkgs\0"));

    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmpkgs %s\x1B[0;%dm%d\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            pkgs,
        );
    }

    c_str(&result)
}

pub fn get_os_name() -> &'static str {
    let result = [0; LEN_STRING + 16];

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
