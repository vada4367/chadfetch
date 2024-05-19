use libc::{c_int, size_t};

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

    pub fn get_color(self, color: c_int) -> c_int {
        match color {
            1 => return self.text as c_int,
            2 => return self.vars as c_int,
            3 => return self.contrast as c_int,
            _ => return 37,
        }
    }
}

pub fn search_palette(key: &str) -> Result<Palette, usize> {
    let palette = ALL_PALETTE.iter().position(|&r| r.0 == key);

    if !palette.is_some() {
        return Err(69);
    }

    Ok(ALL_PALETTE[palette.unwrap()].1)
}

// FOR CREATE YOUR PALETTE
// MAKE CONST WITH NAME WHICH
// YOU WOULD LIKE, AND IN
// ARGUMENTS FOR FUNCTION
// WRITE !ANSI! COLORS
//
//
//
// FIRST ARGUMENT text color:
//
// memory "text"
//           ^
//           |
//
// SECOND ARGUMENT vars color:
//
// memory "text"
//   ^
//   |
//
// LAST ARGUMENT contrast color:
//
// name@hostname
//
//   ^      ^
//   |      |
//
//
// AFTER CREATE YOUR PALETTE
// ADD THIS TO ALL_PALETTE WITH
// NAME

pub const ALL_PALETTE: &[(&str, Palette)] = &[
    ("gigachad\0", GIGACHAD_PALETTE),
    ("openbsd\0", OPENBSD_PALETTE),
    ("void\0", VOID_PALETTE),
];

pub const GIGACHAD_PALETTE: Palette = Palette::new(38, 31, 33);
pub const GENTOO_PALETTE: Palette = Palette::new(37, 35, 94);
pub const VOID_PALETTE: Palette = Palette::new(37, 32, 33);
pub const OPENBSD_PALETTE: Palette = Palette::new(37, 93, 94);
