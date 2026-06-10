use std::env;
use std::path::PathBuf;

fn main() {
    let nozzle_dir = PathBuf::from("nozzle");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // build nozzle static library via cmake (Release to avoid debug CRT on MSVC)
    let nozzle_build = cmake::Config::new(&nozzle_dir)
        .define("NOZZLE_BUILD_TESTS", "OFF")
        .define("NOZZLE_BUILD_EXAMPLES", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .profile("Release")
        .build();

    // link the static library
    println!(
        "cargo:rustc-link-search=native={}/lib",
        nozzle_build.display()
    );
    println!("cargo:rustc-link-lib=static=nozzle");

    // platform-specific linking — must match nozzle CMakeLists.txt
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=IOSurface");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=objc");
        println!("cargo:rustc-link-lib=c++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=drm");
        println!("cargo:rustc-link-lib=gbm");
        println!("cargo:rustc-link-lib=EGL");
        println!("cargo:rustc-link-lib=GL");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=d3d11");
        println!("cargo:rustc-link-lib=dxgi");
        println!("cargo:rustc-link-lib=opengl32");
        println!("cargo:rustc-link-lib=bcrypt");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=user32");
    }

    // generate raw bindings via bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include", nozzle_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    let bindings_path = out_dir.join("nozzle_raw.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("couldn't write bindings");

    // re-run if wrapper.h or nozzle headers change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!(
        "cargo:rerun-if-changed={}/include/nozzle/nozzle_c.h",
        nozzle_dir.display()
    );
}
