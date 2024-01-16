use crate::get_info::*;

pub fn device(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let result = [0; LEN_STRING + 16];
    let spaces_str = utils::spaces(info_space);


    let (name, version);
    let name_str = [0; LEN_STRING];
    let version_str = [0; LEN_STRING];

    unsafe {
        name = fopen(
            c_str("/sys/devices/virtual/dmi/id/product_name\0"),
            c_str("r\0"),
        );
        version = fopen(
            c_str("/sys/devices/virtual/dmi/id/product_version\0"),
            c_str("r\0"),
        );
        if !(name == core::ptr::null_mut()
            || version == core::ptr::null_mut())
        {
            fscanf(name, c_str("%s\n\0"), c_str(&name_str));
            fscanf(version, c_str("%s\n\0"), c_str(&version_str));
        }
    }

    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmhost %s\x1B[0;%dm%s %s\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            c_str(&name_str),
            c_str(&version_str),
        );
    }

    c_str(&result)
}

pub fn uptime(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let result = [0; LEN_STRING + 100];
    let spaces_str = utils::spaces(info_space);

    let file =
        unsafe { fopen(c_str("/proc/uptime\0"), c_str("r\0")) };

    let mut line = [0; LEN_STRING + 30];
    let (mut uptime, mut _uptime) = (0, 0);

    if file != core::ptr::null_mut() {
        unsafe {
            fgets(line.as_mut_ptr(), line.len() as c_int, file);
            sscanf(
                c_str(&line),
                c_str("%d %d\0"),
                &mut uptime,
                &mut _uptime,
            );
        }
    } else { uptime = 0; }

    

    unsafe {
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
    let file =
        unsafe { fopen(c_str("/proc/meminfo\0"), c_str("r\0")) };

    if file == core::ptr::null_mut() {
        unsafe {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("\x1B[0;%dmmemory %s\x1B[0;%dmunknown\0"),
                sys_format.palette.vars,
                spaces_str.as_ptr() as CSTR,
                sys_format.palette.text,
            );

            return c_str(&result);
        }
    }

    let mut line = [0; LEN_STRING + 30];

    let mem_available;
    let (
        mut mem_total,
        mut sh_mem,
        mut mem_free,
        mut buffers,
        mut cached,
        mut s_reclaimable,
    ) = (0, 0, 0, 0, 0, 0);

    while mem_total == 0
        || mem_free == 0
        || buffers == 0
        || cached == 0
        || s_reclaimable == 0
        || sh_mem == 0
    {
        unsafe {
            let fgets_line =
                fgets(line.as_mut_ptr(), line.len() as c_int, file);

            if strstr(c_str(&line), c_str("MemTotal\0"))
                != core::ptr::null_mut()
            {
                sscanf(
                    line.as_ptr() as CSTR,
                    c_str("MemTotal: %d\0"),
                    &mut mem_total,
                );
            }
            if strstr(c_str(&line), c_str("MemFree\0"))
                != core::ptr::null_mut()
            {
                sscanf(
                    line.as_ptr() as CSTR,
                    c_str("MemFree: %d\0"),
                    &mut mem_free,
                );
            }
            if strstr(c_str(&line), c_str("Buffers\0"))
                != core::ptr::null_mut()
            {
                sscanf(
                    line.as_ptr() as CSTR,
                    c_str("Buffers: %d\0"),
                    &mut buffers,
                );
            }
            if strstr(c_str(&line), c_str("Cached\0"))
                != core::ptr::null_mut()
            {
                sscanf(
                    line.as_ptr() as CSTR,
                    c_str("Cached: %d\0"),
                    &mut cached,
                );
            }
            if strstr(c_str(&line), c_str("SReclaimable\0"))
                != core::ptr::null_mut()
            {
                sscanf(
                    line.as_ptr() as CSTR,
                    c_str("SReclaimable: %d\0"),
                    &mut s_reclaimable,
                );
            }
            if strstr(c_str(&line), c_str("Shmem\0"))
                != core::ptr::null_mut()
            {
                sscanf(
                    line.as_ptr() as CSTR,
                    c_str("Shmem: %d\0"),
                    &mut sh_mem,
                );
            }
        }
    }

    mem_available =
        mem_free + buffers + cached + s_reclaimable - sh_mem;

    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmmemory %s\x1B[0;%dm%dM / %dM\0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            (mem_total - mem_available) / 1024,
            mem_total / 1024,
        );
    }

    c_str(&result)
}

pub fn pkgs(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let spaces_str = utils::spaces(info_space);

    let mut distro_pkgs = 0;

    let xbps_pkgs = linux_pkgs::xbps();

    if xbps_pkgs != 0 {
        distro_pkgs = xbps_pkgs;
    }

    let result: [c_char; LEN_STRING] = [0; LEN_STRING];
    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("\x1B[0;%dmpkgs %s\x1B[0;%dm%d \0"),
            sys_format.palette.vars,
            spaces_str.as_ptr() as CSTR,
            sys_format.palette.text,
            distro_pkgs,
        );

        // TODO strcat all pkgs for all distro's like flatpak
    }

    c_str(&result)
}

pub fn get_os_name() -> &'static str {
    let os_release =
        unsafe { fopen(c_str("/etc/os-release\0"), c_str("r\0")) };

    if os_release == core::ptr::null_mut() {
        return "unknown\0";
    }

    let mut os_name = [0; LEN_STRING + 40];
    let os_name_str = unsafe { malloc(40) } as *mut c_char;

    unsafe {
        let fgets_line = fgets(
            os_name.as_mut_ptr(),
            os_name.len() as c_int,
            os_release,
        );

        if strstr(c_str(&os_name), c_str("NAME\0"))
            != core::ptr::null_mut()
        {
            sscanf(
                os_name.as_ptr() as CSTR,
                c_str("NAME=\"%s\"\0"),
                os_name_str,
            );
        }
    }
    return unsafe {
        core::str::from_utf8_unchecked(slice::from_raw_parts(
            os_name_str as *const u8,
            strlen(os_name_str as CSTR) + 1,
        ))
    };
}
