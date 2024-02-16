use bindgen::Builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let lib_path = PathBuf::from_iter(["lib60870", "lib60870-C"]);
    let dst = Config::new(&lib_path)
        .define("BUILD_EXAMPLES", "false")
        .define("BUILD_TESTS", "false")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=lib60870");

    let bindings = Builder::default()
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
