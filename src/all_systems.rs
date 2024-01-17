#![allow(dead_code)]

use crate::logos::*;
use crate::palette::*;

#[derive(Clone, Copy, PartialEq)]
pub enum OS {
    Linux,
    BSD,
    Unknown,
}

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

pub const ALL_SYSTEMS: [SystemFormat<'_>; 3] = [
    SystemFormat::new(
        OS::Linux,
        GIGACHAD_LOGO,
        "Unknown\0",
        VOID_PALETTE,
        0,
    ),
    SystemFormat::new(
        OS::Linux,
        VOID_LOGO,
        "Void\0",
        VOID_PALETTE,
        0,
    ),
    SystemFormat::new(
        OS::BSD,
        GIGACHAD_LOGO,
        "OpenBSD\0",
        VOID_PALETTE,
        0,
    ),
];
