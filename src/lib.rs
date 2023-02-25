#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freetype_init() {
        let mut library = std::ptr::null_mut();
        unsafe {
            FT_Init_FreeType(&mut library);
            FT_Done_FreeType(library);
        }
    }
}