use crate::{
    sys::{FT_Done_Face, FT_Face, FT_New_Memory_Face, FT_Set_Char_Size, FT_Set_Pixel_Sizes},
    FontError, FontLibrary, FontResult,
};

#[derive(Debug)]
pub struct FontFace {
    raw: FT_Face,
}

unsafe impl Send for FontFace {}

impl FontFace {
    pub fn new(library: &FontLibrary, font_data: &[u8]) -> FontResult<Self> {
        unsafe {
            let mut raw = std::ptr::null_mut();
            FontError::from(FT_New_Memory_Face(
                library.raw,
                font_data.as_ptr(),
                font_data.len() as i32,
                0,
                &mut raw,
            ))
            .into_result()
            .map(|_| Self { raw })
        }
    }

    pub fn set_char_size(
        &self,
        char_width: i32,
        char_height: i32,
        horz_resolution: u32,
        vert_resolution: u32,
    ) -> FontResult<()> {
        unsafe {
            FontError::from(FT_Set_Char_Size(
                self.raw,
                char_width,
                char_height,
                horz_resolution,
                vert_resolution,
            ))
            .into_result()
        }
    }

    pub fn set_pixel_sizes(&self, pixel_width: u32, pixel_height: u32) -> FontResult<()> {
        unsafe {
            FontError::from(FT_Set_Pixel_Sizes(self.raw, pixel_width, pixel_height)).into_result()
        }
    }
}

impl Drop for FontFace {
    fn drop(&mut self) {
        unsafe {
            FT_Done_Face(self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_set_char_size() {
        let library = FontLibrary::new().unwrap();
        let face = FontFace::new(&library, include_bytes!("../assets/FiraSans-Bold.ttf")).unwrap();
        face.set_char_size(0, 16 * 64, 300, 300).unwrap();
        drop(face);
        drop(library);
    }

    #[test]
    fn test_face_set_pixel_sizes() {
        let library = FontLibrary::new().unwrap();
        let face = FontFace::new(&library, include_bytes!("../assets/FiraSans-Bold.ttf")).unwrap();
        face.set_pixel_sizes(0, 16).unwrap();
        drop(face);
        drop(library);
    }
}
