use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("freetype");

    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=freetyped");
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_args(&["-I", "freetype/include"])
        .allowlist_function("FT_.*")
        .allowlist_var("FT_.*")
        .allowlist_type("FT_.*")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
