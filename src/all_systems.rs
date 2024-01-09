#![allow(dead_code)]

use libc::size_t;

#[derive(Clone, Copy)]
pub enum OS {
    Linux,
    BSD,
}

#[derive(Clone, Copy)]
pub struct Logo<'a> {
    pub logo: &'a str,
    pub w: size_t,
    pub h: size_t,
}

#[derive(Clone, Copy)]
pub struct SystemFormat<'a> {
    pub os: OS,
    pub logo: Logo<'a>,
    pub name: &'a str,
    pub id: usize,
}

impl SystemFormat<'_> {
    const fn new(
        os: OS,
        logo: &'static str,
        w: size_t,
        h: size_t,
        name: &'static str,
        id: usize,
    ) -> Self {
        SystemFormat {
            os: os,
            logo: Logo {
                logo: logo,
                h: h,
                w: w,
            },
            name: name,
            id: id,
        }
    }
}

pub static ALL_SYSTEMS: [SystemFormat<'_>; 1] = [SystemFormat::new(
    OS::Linux,
    concat!(
        r#"
    _______
 _ \______ -
| \  ___  \ |
| | /   \ | |
| | \___/ | |
| \______ \_|
 -_______\
"#,
        "\0"
    ),
    13,
    7,
    "Void\0",
    0,
)];
