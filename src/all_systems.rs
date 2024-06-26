use crate::get_info::*;
use crate::logos::*;
use crate::palette::*;

// THIS ENUM FOR CHOISE,
// WHERE WE CAN GET DATA
// ABOUT SYSTEM
#[derive(Clone, Copy, PartialEq)]
pub enum OS {
    Linux,
    BSD,
    Unknown,
}

impl OS {
    pub fn get_os_name(&self) -> &'static str {
        match self {
            OS::Linux => {
                return linux::get_os_name();
            }
            OS::BSD => {
                return bsd::get_os_name();
            }
            _ => {
                return "unknown_os\0";
            }
        }
    }
}

// MAIN STRUCT
#[derive(Clone, Copy)]
pub struct SystemFormat<'a> {
    pub os: OS,
    pub logo: Logo<'a>,
    pub name: &'static str,
    pub palette: Palette,
    pub id: usize,
}

impl SystemFormat<'_> {
    const fn new<'a>(
        os: OS,
        logo: Logo<'a>,
        name: &'static str,
        palette: Palette,
        id: usize,
    ) -> SystemFormat<'a> {
        SystemFormat {
            os: os,
            logo: logo,
            name: name,
            palette: palette,
            id: id,
        }
    }
}

// README!!!
//
// IF YOU WILL ADD YOUR SYSTEM:
//
// In 1st argument write enum OS
// In 2nd LOGO (FROM src/logos.rs)
// In 3 first string of /etc/os-release
// In 4 Palette (FROM src/palette.rs)
// In 5 JUST NUMBER IN ARRAY
//
// GOOD LUCK!

pub const ALL_SYSTEMS: [SystemFormat<'_>; 4] = [
    SystemFormat::new(OS::Linux, GENTOO_LOGO, "Gentoo\0", GENTOO_PALETTE, 0),
    SystemFormat::new(
        OS::Linux,
        GIGACHAD_LOGO,
        "GigachadOS (unknown)\0",
        GIGACHAD_PALETTE,
        0,
    ),
    SystemFormat::new(OS::Linux, VOID_LOGO, "Void\0", VOID_PALETTE, 0),
    SystemFormat::new(OS::BSD, OPENBSD_LOGO, "OpenBSD\0", OPENBSD_PALETTE, 0),
];
