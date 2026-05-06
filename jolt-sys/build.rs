use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg = cmake::Config::new("./");

    let profile = match &std::env::var("PROFILE").unwrap()[..] {
        "debug" => "Debug",
        "release" | "bench" => "Release",
        unknown => {
            eprintln!(
                "Warning: unknown Rust profile={}; defaulting to a release build.",
                unknown
            );
            "Release"
        }
    };

    let enable_debug_renderer = env::var("CARGO_FEATURE_DEBUG_RENDERER").is_ok();

    let dst = cfg
        .define("ENABLE_ALL_WARNINGS", "OFF")
        .define("USE_STATIC_MSVC_RUNTIME_LIBRARY", "OFF")
        .define("INTERPROCEDURAL_OPTIMIZATION", "OFF")
        .define(
            "DEBUG_RENDERER_IN_DISTRIBUTION",
            if enable_debug_renderer { "ON" } else { "OFF" },
        )
        .define(
            "DEBUG_RENDERER_IN_DEBUG_AND_RELEASE",
            if enable_debug_renderer { "ON" } else { "OFF" },
        )
        // .build_target("JoltC")
        .profile(profile)
        .build_target("JoltC")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build/").join(profile).display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build/").display()
    );
    println!("cargo:rustc-link-lib=JoltC");
    println!("cargo:rustc-link-lib=Jolt");

    let mut bindgen = bindgen::Builder::default()
        .header("JoltC/JoltPhysicsC.h")
        .allowlist_item("JPC_+.*");

    if enable_debug_renderer {
        bindgen = bindgen.clang_args(["-D", "JPH_DEBUG_RENDERER=1"]);
    }

    let bindings = bindgen
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
    println!("cargo:rerun-if-changed=CMakeLists.txt");
}
