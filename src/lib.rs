pub mod error;
pub mod face;
pub mod library;

pub use error::{FontError, FontResult};
pub use face::FontFace;
pub use library::FontLibrary;

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
