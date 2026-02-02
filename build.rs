use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    // Output directory for generated files
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Paths
    let wrapper_dir = PathBuf::from("wrapper");
    let wrapper_cxsparse = wrapper_dir.join("suitesparse_wrapper.c");
    let wrapper_umfpack = wrapper_dir.join("umfpack_wrapper.c");
    let wrapper_hdrs = wrapper_dir.clone();
    let suitesparse_dir = wrapper_dir.join("suitesparse");

    let macos_target = env::var("MACOSX_DEPLOYMENT_TARGET").unwrap_or("13.0".into());

    // Build OpenBLAS
    let openblas_dst = Config::new(wrapper_dir.join("openblas"))
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("USE_OPENMP", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", &macos_target)
        .build();

    let openblas_lib = openblas_dst.join("lib");
    let openblas_include = openblas_dst.join("include/openblas");

    println!("cargo:rustc-link-search=native={}", openblas_lib.display());
    println!("cargo:rustc-link-lib=static=openblas");

    // On Linux, link math library
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=m");
    }

    // Build SuiteSparse
    let suitesparse_dst = Config::new(&suitesparse_dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("SUITESPARSE_ENABLE_PROJECTS", "umfpack;cxsparse;amd;colamd;cholmod;ccolamd;camd")
        .define("BLAS_LIBRARIES", openblas_lib.join("libopenblas.a").to_str().unwrap())
        .define("LAPACK_LIBRARIES", openblas_lib.join("libopenblas.a").to_str().unwrap())
        .define("BLAS_INCLUDE_DIRS", openblas_include.to_str().unwrap())
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", &macos_target)
        .define("SUITESPARSE_USE_FORTRAN", "OFF")
        .build();

    let ss_lib = suitesparse_dst.join("lib");
    let ss_include = suitesparse_dst.join("include/suitesparse");

    println!("cargo:rustc-link-search=native={}", ss_lib.display());
    println!("cargo:rustc-link-lib=static=umfpack");
    println!("cargo:rustc-link-lib=static=cxsparse");
    println!("cargo:rustc-link-lib=static=amd");
    println!("cargo:rustc-link-lib=static=colamd");
    println!("cargo:rustc-link-lib=static=cholmod");
    println!("cargo:rustc-link-lib=static=ccolamd");
    println!("cargo:rustc-link-lib=static=camd");
    println!("cargo:rustc-link-lib=static=suitesparseconfig");

    // Compile C wrappers
    cc::Build::new()
        .files([wrapper_cxsparse.clone(), wrapper_umfpack.clone()])
        .include(&ss_include)
        .include(&openblas_include)
        .include(&wrapper_hdrs)
        .compile("suitesparse_wrappers");

    // Generate Rust bindings
    bindgen::Builder::default()
        .header(wrapper_dir.join("suitesparse_wrapper.h").to_str().unwrap())
        .header(wrapper_dir.join("umfpack_wrapper.h").to_str().unwrap())
        .clang_arg(format!("-I{}", ss_include.display()))
        .clang_arg(format!("-I{}", openblas_include.display()))
        .allowlist_function(".*") // allow all wrapper functions
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("suitesparse_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Trigger rebuilds if sources change
    println!("cargo:rerun-if-changed=wrapper/openblas");
    println!("cargo:rerun-if-changed=wrapper/suitesparse");
    println!("cargo:rerun-if-changed={}", wrapper_cxsparse.display());
    println!("cargo:rerun-if-changed={}", wrapper_umfpack.display());
    println!("cargo:rerun-if-changed={}", wrapper_hdrs.display());
}
