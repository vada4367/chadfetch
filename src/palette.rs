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

pub const VOID_PALETTE: Palette = Palette::new(37, 32, 33);
pub const OPENBSD_PALETTE: Palette = Palette::new(37, 33, 34);
