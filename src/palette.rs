use libc::size_t;

#[derive(Clone, Copy)]
pub struct Palette {
    pub text: size_t,
    pub vars: size_t,
    pub contrast: size_t,
}

impl Palette {
    pub const fn new(c1: size_t, c2: size_t, c3: size_t) -> Self {
        Self {
            text: c1,
            vars: c2,
            contrast: c3,
        }
    }
}

pub const VOID_PALETTE: Palette = Palette::new(37, 32, 33);