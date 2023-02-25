use std::fmt::Display;

use libc::{c_char, strlen};
use thiserror::Error;

use crate::sys::{FT_Err_Ok, FT_Error_String};

#[derive(Debug, Error)]
pub struct FontError(i32, String);

impl FontError {
    pub fn is_ok(&self) -> bool {
        self.0 == FT_Err_Ok
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn into_result(self) -> Result<(), FontError> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "FreeType returned error {}: {}",
            self.0, self.1
        ))
    }
}

impl From<i32> for FontError {
    fn from(error: i32) -> Self {
        unsafe {
            let error_raw_str = FT_Error_String(error) as *const c_char;
            if error_raw_str.is_null() {
                FontError(error, "No error message".into())
            } else {
                let error_slice =
                    std::slice::from_raw_parts(error_raw_str as *const u8, strlen(error_raw_str));
                let error_str = String::from_utf8_unchecked(error_slice.into());
                FontError(error, error_str)
            }
        }
    }
}

pub type FontResult<T> = Result<T, FontError>;
