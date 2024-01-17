use libc::size_t;

#[derive(Clone, Copy)]
pub struct Logo<'a> {
    pub logo: &'a str,
    pub w: size_t,
    pub h: size_t,
}

impl Logo<'_> {
    const fn new(logo: &'static str, w: size_t, h: size_t) -> Self {
        Logo {
            logo: logo,
            w: w,
            h: h,
        }
    }
}

pub const UNKNOWN_LINUX_LOGO: Logo<'_> = Logo::new(
    concat!(
        r#"
     ___ 
    (.. |
    (<> |
   / __  \
  ( /  \ /|
 _/\ __)/_)
 \/-____\/
"#,
        "\0"
    ),
    13,
    7,
);

pub const OPENBSD_LOGO: Logo<'_> = Logo::new(
    concat!(
        r#"
     ${c1}_____
   \-     -/
\_/         \
|        O O |
|_  <   )  3 )
/ \         /
   /-_____-\
"#,
        "\0"
    ),
    13,
    7,
);

pub const VOID_LOGO: Logo<'_> = Logo::new(
    concat!(
        r#"
    ${c3}_______
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
);
