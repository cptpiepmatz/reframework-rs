use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=deps/REFramework/include/reframework/API.h");
    println!("cargo:rerun-if-changed=deps/REFramework/include/reframework/API.hpp");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("deps/REFramework/include/reframework/API.h")
        //.clang_arg("-std=c++17")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
