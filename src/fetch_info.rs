#[derive(Clone, Copy)]
pub struct FetchInfo {
    pub logo: bool,
    pub user_host: bool,
    pub os: bool,
    pub device: bool,
    pub kernel: bool,
    pub uptime: bool,
}
