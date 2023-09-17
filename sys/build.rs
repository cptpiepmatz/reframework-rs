use bindgen::callbacks::ParseCallbacks;
use std::env;
use std::fmt::Debug;
use std::path::PathBuf;

const HEADER_FILE: &str = include_str!("deps/REFramework/include/reframework/API.h");

fn main() {
    println!("cargo:rerun-if-changed=deps/REFramework/include/reframework/API.h");
    println!("cargo:rerun-if-changed=deps/REFramework/include/reframework/API.hpp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build.toml");

    let bindings = bindgen::Builder::default()
        .header_contents("API.h", HEADER_FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(CustomCallbacks))
        .clang_arg("-fparse-all-comments")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}

static_toml::static_toml! {
    static BUILD = include_toml!("build.toml");
}

#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        for excluded_comment in BUILD.excluded_comments {
            if excluded_comment.trim() == comment.trim() {
                return Some(String::new());
            }
        }

        None
    }
}
