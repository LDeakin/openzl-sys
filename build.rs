// fn get_openzl_version() -> (u32, u32, u32) {
//     let version = env!("CARGO_PKG_VERSION");
//     let versions = version.split("+openzl.").collect::<Vec<_>>();
//     let openzl_version = match &versions[..] {
//         &[_crate_version, openzl_version] => openzl_version,
//         _ => panic!("Could not identify openzl version from crate version"),
//     };
//     let openzl_versions = openzl_version.split(".").collect::<Vec<_>>();
//     match &openzl_versions[..] {
//         &[major, minor, patch] => (
//             major
//                 .parse::<u32>()
//                 .expect("OpenZL major version is not an integer"),
//             minor
//                 .parse::<u32>()
//                 .expect("OpenZL minor version is not an integer"),
//             patch
//                 .parse::<u32>()
//                 .expect("OpenZL patch version is not an integer"),
//         ),
//         _ => panic!("Could not identify openzl version from crate version"),
//     }
// }

fn compile_openzl_cc() {
    // Get zstd include path from zstd-sys
    let zstd_include =
        std::env::var("DEP_ZSTD_INCLUDE").expect("DEP_ZSTD_INCLUDE not set by zstd-sys");

    let mut build = cc::Build::new();
    build.std("c11");
    build.include("openzl/include");
    build.include("openzl/src");
    build.include(&zstd_include);
    build.define("ZSTD_DISABLE_ASM", "1");
    // Redefine ZL_INLINE to remove 'inline' keyword, making inline functions
    // available as actual symbols that can be linked
    build.define("ZL_INLINE", "static ZL_UNUSED_ATTR");
    // Suppress warnings from C compilation
    build.flag("-w"); // Suppress all warnings

    let src_files = glob::glob("openzl/src/**/*.c")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok);
    for src in src_files {
        build.file(src);
    }

    build.compile("openzl");
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("openzl/include/openzl/openzl.h")
        .clang_arg("-Iopenzl/include")
        // Redefine ZL_INLINE to remove 'inline' keyword so bindgen generates
        // bindings for inline functions like ZL_compressBound
        .clang_arg("-DZL_INLINE=static ZL_UNUSED_ATTR")
        // Enable experimental wrapper for static inline functions
        .wrap_static_fns(true)
        .wrap_static_fns_path(manifest_dir.join("static_fns_wrapper"))
        .allowlist_item("ZL_.*")
        .anon_fields_prefix("field_")
        .prepend_enum_name(false) // Variants already have the enum name as prefix
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rust_target(unsafe { bindgen::RustTarget::stable(63, 0).unwrap_unchecked() })
        .generate()
        .expect("Unable to generate bindings");

    let out_path = manifest_dir.join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .unwrap_or_else(|_| panic!("Couldn't write bindings to {out_path:?}!"));
}

fn main() {
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=openzl");
    println!("cargo:rustc-link-lib=static=openzl");

    let dst = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    compile_openzl_cc();

    #[cfg(feature = "bindgen")]
    generate_bindings();

    // Compile the static inline function wrapper if it exists
    let wrapper_c = manifest_dir.join("static_fns_wrapper.c");
    let mut build = cc::Build::new();
    build.std("c11");
    build.include("openzl/include");
    build.file(&wrapper_c);
    // Suppress warnings from C compilation
    build.flag("-w"); // Suppress all warnings
    build.compile("static_fns_wrapper");

    std::fs::create_dir_all(dst.join("include")).unwrap();
    let header_files = glob::glob("openzl/include/**/*.h").unwrap();
    for header in header_files {
        let header = header.unwrap();
        let relative_path = header.strip_prefix("openzl/include").unwrap();
        let dest_path = dst.join("include").join(relative_path);
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::copy(&header, &dest_path).unwrap();
    }

    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}/include", dst.to_str().unwrap());
}
