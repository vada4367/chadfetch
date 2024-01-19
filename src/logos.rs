#![allow(dead_code)]
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

// FOR CREATE YOUR LOGO,
// YOU WRITE LOGO TEXT
// AND WIDTH & HEIGHT
// IN CONST WITH NAME
// WHICH YOU WOULD LIKE
//
// AND ADD THIS TO ALL_LOGO
// WITH NAME

pub const ALL_LOGO: &[(&str, Logo<'_>)] = &[
    ("gigachad\0", GIGACHAD_LOGO),
    ("openbsd\0", OPENBSD_LOGO),
    ("void\0", VOID_LOGO),
];

pub fn search_logo(key: &str) -> Logo<'_> {
    ALL_LOGO
        .binary_search_by(|(k, _)| k.cmp(&key))
        .map(|x| ALL_LOGO[x].1)
        .expect("NO LOGO")
}

pub const GIGACHAD_LOGO: Logo<'_> = Logo::new(
    concat!(
        r#"
     ________
    VT       FGV
    U ${c2}CHAD${c1}     AV
   ${c2}FETCH${c1}  RL    U
   |  #    YKSA
   T-IIA ${c2}sAFE${c1} A U
   \  ___ T /--/
    L*-=^ ^#/| \
    AMPERSAND .W\
     %#####% .;i&} 
"#,
        "\0"
    ),
    17,
    10,
);

pub const OPENBSD_LOGO: Logo<'_> = Logo::new(
    concat!(
        r#"
     ${c2}_____
   \-     -/
\_/         \
|        ${c1}O O${c2} |
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
    ${c2}_______
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
