use crate::{
    sys::{FT_Done_FreeType, FT_Init_FreeType, FT_Library},
    FontError, FontResult,
};

#[derive(Debug)]
pub struct FontLibrary {
    pub(crate) raw: FT_Library,
}

unsafe impl Send for FontLibrary {}

impl FontLibrary {
    pub fn new() -> FontResult<Self> {
        unsafe {
            let mut raw = std::ptr::null_mut();
            FontError::from(FT_Init_FreeType(&mut raw))
                .into_result()
                .map(|_| Self { raw })
        }
    }
}

impl Drop for FontLibrary {
    fn drop(&mut self) {
        unsafe {
            FT_Done_FreeType(self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_init() {
        let library = FontLibrary::new().unwrap();
        drop(library)
    }
}
