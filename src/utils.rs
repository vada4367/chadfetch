use crate::{FetchInfo, SystemFormat};
use libc::c_int;
use crate::libc::*;
use ::libc::{c_char, size_t};
use core::slice;

pub const LEN_STRING: usize = 1;


impl SystemFormat<'_> {
    pub fn print_fetch(&self, settings: FetchInfo) {
        unsafe {
            let mut dy = -2;

            if settings.logo {
                dy = self.logo.h as i32;

                let logo_chars = slice::from_raw_parts(
                    c_str(self.logo.logo) as *const u8,
                    strlen(c_str(self.logo.logo)) + 1,
                );

                let mut i: usize = 0;
                let mut checking = false;

                printf(c_str("\x1B[%dA\0"), 1);
                while logo_chars[i] != 0 {
                    if logo_chars[i] == b'$' {
                        checking = true;
                    }
                    if checking
                        && (logo_chars[i] as c_int > 48)
                        && (logo_chars[i] as c_int) < 52
                    {
                        printf(
                            c_str("\x1B[0;%dm\0"),
                            self.palette.get_color(logo_chars[i] as c_int - 48),
                        );
                    }
                    if !checking {
                        printf(
                            c_str("%c\0"),
                            logo_chars[i] as c_int,
                        );
                    }
                    if checking && logo_chars[i] == b'}' {
                        checking = false;
                    }

                    i += 1;
                }
            }

            // MOVE THE CURSOR TO
            // THE BEGGINNING OF
            // THE OUTPUT
            printf(c_str("\x1B[%dA\0"), dy);

            // DY IS LEN (Y) OF LOGO
            dy -= self.print_all_info(settings);

            // MOVE THE CURSOR TO
            // THE END OF THE OUTPUT
            printf(c_str("\x1B[%dB\0"), dy - 1);
            printf(c_str("\n\0"));
        }
    }

    // INFO IS THE LINES AFTER LOGO
    fn print_info(info: CSTR, space: i32) {
        unsafe {
            // MOVE CURSOR TO END OF LOGO (X)
            printf(c_str("\x1B[%dC\0"), space + 4);

            printf(c_str("%s\n\0"), info);
        }
    }

    fn print_all_info(&self, settings: FetchInfo) -> i32 {
        let mut print_space = -4;

        // PRINT_SPACE IS VARIABLE FOR
        // MAKE PLACE DATA STRINGS AFTER
        // LOGO
        if settings.logo {
            print_space = self.logo.w as i32;
        }

        // THIS VAR NEEDS TO MOVE CURSOR
        // TO THE END OF OUTPUT
        let mut count_of_info = 0;

        if settings.user_host {
            Self::print_info(self.user_host(), print_space);
            count_of_info += 1;
        }

        // MAX_LENGTH NEEDS TO MAKE A CORRECT
        // SPACES (ALL INFO ON ONE "Y" LINE)
        let max_length = settings.max_length();

        if settings.os {
            Self::print_info(self.os(max_length - 2), print_space);
            count_of_info += 1;
        }
        if settings.device {
            Self::print_info(
                self.device(max_length - 4),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.kernel {
            Self::print_info(
                self.kernel(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.uptime {
            Self::print_info(
                self.uptime(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }
        if settings.pkgs {
            Self::print_info(self.pkgs(max_length - 4), print_space);
            count_of_info += 1;
        }
        if settings.memory {
            Self::print_info(
                self.memory(max_length - 6),
                print_space,
            );
            count_of_info += 1;
        }

        count_of_info
    }
}


pub fn spaces(info_space: size_t) -> [c_char; 20] {
    let mut spaces = [0x20 as c_char; 20];
    spaces[info_space] = 0 as c_char;

    spaces
}

pub fn time(secs: size_t) -> CSTR {
    let result = [0; LEN_STRING + 16];

    unsafe {
        if secs / 86400 != 0 {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%dd %dh %dm\0"),
                secs / 86400,
                secs % 86400 / 3600,
                secs % 3600 / 60,
            );
        } else if secs % 86400 / 3600 != 0 {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%dh %dm\0"),
                secs / 3600,
                secs % 3600 / 60,
            );
        } else {
            sprintf(
                result.as_ptr() as *mut c_char,
                c_str("%dm\0"),
                secs / 60,
            );
        }
    }

    result.as_ptr() as CSTR
}

macro_rules! delete_char {
    ($string:expr, $char:expr) => {{
        let mut p = $string;
        loop {
            p = unsafe { strchr(p, $char) };
            if p.is_null() {
                break;
            }
            unsafe { strcpy(p as *mut c_char, p.add(1)) };
        }
    }};
}
pub(crate) use delete_char;
