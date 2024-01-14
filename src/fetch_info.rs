use libc::size_t;

#[derive(Clone, Copy)]
pub struct Colors {
    pub main: size_t,
    pub colorful: size_t,
    pub colorless: size_t,
}

impl Colors {
    pub fn new(c1: size_t, c2: size_t, c3: size_t) -> Self {
        Colors {
            main: c1,
            colorful: c2,
            colorless: c3,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FetchInfo {
    pub logo: bool,
    pub user_host: bool,
    pub os: bool,
    pub device: bool,
    pub kernel: bool,
    pub uptime: bool,
    pub pkgs: bool,
    pub memory: bool,
    pub colors: Colors,
}

impl FetchInfo {
    pub fn max_length(self) -> size_t {
        if self.kernel || self.uptime || self.memory {
            return 6;
        }
        if self.pkgs || self.device {
            return 4;
        }
        return 2;
    }
}
