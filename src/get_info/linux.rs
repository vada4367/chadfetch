use crate::get_info::*;
pub fn device(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let (name, version);

    unsafe {
        name = fopen(
            c_str("/sys/devices/virtual/dmi/id/product_name\0"),
            c_str("r\0"),
        );
        version = fopen(
            c_str("/sys/devices/virtual/dmi/id/product_version\0"),
            c_str("r\0"),
        );
        if name == core::ptr::null_mut()
            || version == core::ptr::null_mut()
        {
            let result = [0; LEN_STRING];

            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("host %sunknown\0"),
                spaces_str.as_ptr() as CSTR,
            );

            return c_str(&result);
        }
    }

    let result = [0; LEN_STRING];
    let name_str = [0; LEN_STRING];
    let version_str = [0; LEN_STRING];

    unsafe {
        fscanf(name, c_str("%s\n\0"), c_str(&name_str));
        fscanf(version, c_str("%s\n\0"), c_str(&version_str));
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("host %s%s %s\0"),
            spaces_str.as_ptr() as CSTR,
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
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let file =
        unsafe { fopen(c_str("/proc/uptime\0"), c_str("r\0")) };
    let result = [0; LEN_STRING + 100];

    if file == core::ptr::null_mut() {
        unsafe {
            strcat(
                result.as_ptr() as *mut c_char,
                spaces_str.as_ptr() as CSTR,
            );

            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("uptime %sunknown\0"),
                spaces_str,
            );

            return c_str(&result);
        }
    }

    let mut line = [0; LEN_STRING + 30];
    let (mut uptime, mut _uptime) = (0, 0);

    unsafe {
        let fgets_line =
            fgets(line.as_mut_ptr(), line.len() as c_int, file);
        sscanf(
            c_str(&line),
            c_str("%d %d\0"),
            &mut uptime,
            &mut _uptime,
        );
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

    let file =
        unsafe { fopen(c_str("/proc/meminfo\0"), c_str("r\0")) };

    if file == core::ptr::null_mut() {
        unsafe {
            let result = [0; LEN_STRING];

            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("memory %sunknown\0"),
                spaces_str.as_ptr() as CSTR,
            );

            return c_str(&result);
        }
    }

    let mut line = [0; LEN_STRING + 30];

    let mem_available;
    let mut mem_total = 0;
    let mut sh_mem = 0;
    let mut mem_free = 0;
    let mut buffers = 0;
    let mut cached = 0;
    let mut s_reclaimable = 0;

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

    let result = [0; LEN_STRING];
    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("memory %s%dM / %dM\0"),
            spaces_str.as_ptr() as CSTR,
            (mem_total - mem_available) / 1024,
            mem_total / 1024,
        );
    }

    c_str(&result)
}

fn xbps() -> size_t {
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

pub fn pkgs(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let mut distro_pkgs = 0;

    let xbps_pkgs = xbps();

    if xbps_pkgs != 0 {
        distro_pkgs = xbps_pkgs;
    }

    let result: [c_char; LEN_STRING] = [0; LEN_STRING];
    unsafe {
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("pkgs %s%d \0"),
            spaces_str.as_ptr() as CSTR,
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
