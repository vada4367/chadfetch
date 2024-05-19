use crate::get_info::*;

pub fn xbps() -> size_t {
    let path = utils::full_path(c_str("/var/db/xbps/\0"), c_str("pkgdb\0"));

    if path.is_null() {
        return 0;
    }

    let installed_string = c_str("<string>installed</string>\0");

    let mut raw_file;
    unsafe {
        let f = fopen(path, c_str("r\0"));

        if f.is_null() {
            return 0;
        }

        let mut stat = MaybeUninit::<stat_struct>::uninit().assume_init();

        stat_func(path, &mut stat);
        raw_file = malloc(stat.st_size as usize);
        fread(raw_file, 1, stat.st_size as size_t, f);
    }

    let mut count = 0;
    unsafe {
        loop {
            raw_file = strstr(raw_file as CSTR, installed_string) as *mut c_void;
            if raw_file.is_null() {
                break;
            }
            count += 1;
            raw_file = raw_file.add(strlen(installed_string));
        }
    }

    count
}

pub fn pacman() -> size_t {
    let pacman_dir = c_str("/var/lib/pacman/local\0");
    let pkgs = unix::search_pkgs(pacman_dir);

    pkgs
}

pub fn emerge() -> size_t {
    let pkgdb_dir = c_str("/var/db/pkg/\0");
    let pkgs = unix::search_pkgs_deeply(pkgdb_dir);

    pkgs
}
