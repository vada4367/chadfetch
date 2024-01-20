use libc::size_t;

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
}

//
// FUNCTION max_length FOR
// MAKE RIGHT SPACES NAME AND
// DATA. Example:
//
// os <-----> Linux
// uptime <-> 3h 20m
//
impl FetchInfo {
    pub fn get_full_info() -> Self {
        Self {
            logo: true,
            user_host: true,
            os: true,
            device: true,
            kernel: true,
            uptime: true,
            pkgs: true,
            memory: true,
        }
    }

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
