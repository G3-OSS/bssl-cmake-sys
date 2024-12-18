use std::env;
use std::path::Path;

use bindgen::MacroTypeVariation;

// Keep in sync with the list in include/openssl/opensslconf.h
const OSSL_CONF_DEFINES: &[&str] = &[
    "OPENSSL_NO_ASYNC",
    "OPENSSL_NO_BF",
    "OPENSSL_NO_BLAKE2",
    "OPENSSL_NO_BUF_FREELISTS",
    "OPENSSL_NO_CAMELLIA",
    "OPENSSL_NO_CAPIENG",
    "OPENSSL_NO_CAST",
    "OPENSSL_NO_CMS",
    "OPENSSL_NO_COMP",
    "OPENSSL_NO_CT",
    "OPENSSL_NO_DANE",
    "OPENSSL_NO_DEPRECATED",
    "OPENSSL_NO_DGRAM",
    "OPENSSL_NO_DYNAMIC_ENGINE",
    "OPENSSL_NO_EC_NISTP_64_GCC_128",
    "OPENSSL_NO_EC2M",
    "OPENSSL_NO_EGD",
    "OPENSSL_NO_ENGINE",
    "OPENSSL_NO_GMP",
    "OPENSSL_NO_GOST",
    "OPENSSL_NO_HEARTBEATS",
    "OPENSSL_NO_HW",
    "OPENSSL_NO_IDEA",
    "OPENSSL_NO_JPAKE",
    "OPENSSL_NO_KRB5",
    "OPENSSL_NO_MD2",
    "OPENSSL_NO_MDC2",
    "OPENSSL_NO_OCB",
    "OPENSSL_NO_OCSP",
    "OPENSSL_NO_RC2",
    "OPENSSL_NO_RC5",
    "OPENSSL_NO_RFC3779",
    "OPENSSL_NO_RIPEMD",
    "OPENSSL_NO_RMD160",
    "OPENSSL_NO_SCTP",
    "OPENSSL_NO_SEED",
    "OPENSSL_NO_SM2",
    "OPENSSL_NO_SM3",
    "OPENSSL_NO_SM4",
    "OPENSSL_NO_SRP",
    "OPENSSL_NO_SSL_TRACE",
    "OPENSSL_NO_SSL2",
    "OPENSSL_NO_SSL3",
    "OPENSSL_NO_SSL3_METHOD",
    "OPENSSL_NO_STATIC_ENGINE",
    "OPENSSL_NO_STORE",
    "OPENSSL_NO_WHIRLPOOL",
];

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&crate_dir);
    let boringssl_src_dir = src_dir.join("third_party").join("boringssl");
    let include_dir = boringssl_src_dir.join("src").join("include");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let bindgen_file = out_dir.join("bindgen.rs");

    let wrapper_header_src = src_dir.join("wrapper.h");
    let wrapper_header_dst = out_dir.join("wrapper.h");
    std::fs::copy(wrapper_header_src, wrapper_header_dst).unwrap();
    let wrapper_source = out_dir.join("wrapper.c");

    let target = env::var("TARGET").unwrap();

    #[cfg(not(windows))]
    link_cxx_runtime();

    // bindgen
    let binding = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(false)
        .enable_function_attribute_detection()
        .wrap_static_fns(true)
        .wrap_static_fns_path(&wrapper_source)
        .use_core()
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .rustified_enum("point_conversion_form_t")
        .allowlist_file(".*[[:punct:]]include[[:punct:]]openssl[[:punct:]].*\\.h")
        .clang_args([
            format!("-I{}", include_dir.display()),
            format!("--target={target}"),
        ])
        .generate()
        .expect("unable to generate binding for BoringSSL");
    binding
        .write_to_file(&bindgen_file)
        .expect("failed to write bindgen file");

    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=wrapper.h");
    cc::Build::new()
        .cargo_metadata(true)
        .include(&include_dir)
        .file(wrapper_source)
        .compile("rustc_wrapper");

    // build BoringSSL code
    println!("cargo:rerun-if-changed={}", boringssl_src_dir.display());
    let mut cmake_config = cmake::Config::new(boringssl_src_dir);
    cmake_config.build_target("crypto").build_target("ssl");
    #[cfg(windows)]
    cmake_config.generator("Ninja");
    #[cfg(target_env = "msvc")]
    select_msvc_crt(&mut cmake_config);
    let boringssl_build_dir = cmake_config.build();

    let lib_search_dir = Path::new(&boringssl_build_dir).join("build");
    // set link options
    println!(
        "cargo:rustc-link-search=native={}",
        lib_search_dir.display()
    );
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");

    // OSSL CONF
    println!("cargo:conf={}", OSSL_CONF_DEFINES.join(","));
}

#[cfg(not(windows))]
fn link_cxx_runtime() {
    // libssl requires a C++ runtime, such as libstdc++ or libc++
    println!("cargo:rerun-if-changed=link_runtime.cpp");
    cc::Build::new()
        .cargo_metadata(true)
        .cpp(true)
        .file("link_runtime.cpp")
        .compile("link_runtime");
}

#[cfg(target_env = "msvc")]
fn select_msvc_crt(cmake_config: &mut cmake::Config) {
    let linkage = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    if linkage.contains("crt-static") {
        cmake_config.define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreaded");
    } else {
        cmake_config.define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");
    }
}
