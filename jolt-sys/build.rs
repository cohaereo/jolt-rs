use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg = cmake::Config::new("./");

    let profile = cfg.get_profile().to_string();

    let dst = cfg
        .define("ENABLE_ALL_WARNINGS", "OFF")
        .define("USE_STATIC_MSVC_RUNTIME_LIBRARY", "OFF")
        // .build_target("JoltC")
        .build_target("ALL_BUILD")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build/").join(profile).display()
    );
    println!("cargo:rustc-link-lib=Jolt");
    println!("cargo:rustc-link-lib=JoltC");

    let bindings = bindgen::Builder::default()
        .header("JoltC/JoltPhysicsC.h")
        .allowlist_item("JPC_+.*")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=JoltC/JoltPhysicsC.h");
}
