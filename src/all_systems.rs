#![allow(dead_code)]

use crate::logos::*;

#[derive(Clone, Copy)]
pub enum OS {
    Linux,
    BSD,
}

#[derive(Clone, Copy)]
pub struct SystemFormat<'a> {
    pub os: OS,
    pub logo: Logo<'a>,
    pub name: &'static str,
    pub id: usize,
}

impl SystemFormat<'_> {
    const fn new<'a>(
        os: OS,
        logo: Logo<'a>,
        name: &'static str,
        id: usize,
    ) -> SystemFormat<'a> {
        SystemFormat {
            os: os,
            logo: logo,
            name: name,
            id: id,
        }
    }
}

pub const ALL_SYSTEMS: [SystemFormat<'_>; 2] = [
    SystemFormat::new(OS::Linux, UNKNOWN_LOGO, "Unknown\0", 0),
    SystemFormat::new(OS::Linux, VOID_LOGO, "Void\0", 0),
];
