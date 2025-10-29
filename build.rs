extern crate autotools;
extern crate bindgen;
extern crate fs_extra;

use autotools::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut libunwind_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    libunwind_path.push("libunwind");
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _e = fs_extra::dir::create(libunwind_path.clone(), true);
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::dir::copy(project_dir.join("libunwind"), out_dir.clone(), &options).unwrap();

    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    // Choose build.
    let link_lib_arch = match target.as_str() {
        "x86_64-unknown-linux-gnu" | "x86_64-unknown-linux-musl" => "x86_64",
        "i686-unknown-linux-gnu" | "i586-unknown-linux-gnu" => "x86",
        "arm-unknown-linux-gnueabihf" => "arm",
        _ => "",
    };
    if link_lib_arch.is_empty() {
        println!("cargo:warning=target {} is unsupported", target);
        return;
    }

    // Build libunwind.
    let _autogen = Command::new("sh")
        .current_dir(&libunwind_path)
        .arg("-c")
        .arg(format!(
            "autoreconf --force --install --verbose {}",
            &libunwind_path.to_str().unwrap()
        ))
        .output()
        .expect("failed to run autoreconf, do you have the autotools installed?");

    // Configure. Check if we compile for  x86 target on x86_64 host.
    let mut dst = Config::new(&libunwind_path);
    if env::var_os("CARGO_FEATURE_PTRACE").is_none() {
        dst.disable("ptrace", None);
    } else {
        println!("cargo:warning=ptrace-on");
        dst.enable("ptrace", None);
    }

    dst.disable("documentation", None)
        .disable("tests", None)
        .enable_shared()
        .enable_static();

    let dst = dst.build();
    println!("cargo:rustc-link-search={}/lib", dst.display());

    if target.contains("musl") {
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-lib=static=lzma");
        println!("cargo:rustc-link-lib=static=unwind-{}", link_lib_arch);
        println!("cargo:rustc-link-lib=static=unwind");
        println!("cargo:rustc-link-lib=static=unwind-coredump");

        if env::var_os("CARGO_FEATURE_PTRACE").is_some() {
            println!("cargo:rustc-link-lib=static=unwind-ptrace");
        }
    } else {
        println!("cargo:rustc-link-lib=unwind-{}", link_lib_arch);
        println!("cargo:rustc-link-lib=unwind");
        println!("cargo:rustc-link-lib=unwind-coredump");

        if env::var_os("CARGO_FEATURE_PTRACE").is_some() {
            println!("cargo:rustc-link-lib=unwind-ptrace");
        }
    }

    // Choose header.
    let wrapper = if link_lib_arch == "arm" && host.contains("x86_64") {
        "wrapper-arm.h"
    } else {
        "wrapper.h"
    };
    let include_opt = format!("-I{}/include", out_dir.to_str().unwrap());
    let bindings = match link_lib_arch {
        "x86" => bindgen::Builder::default()
            .header(project_dir.join(wrapper).to_str().unwrap())
            .clang_arg(&include_opt)
            .blocklist_function("_Ux86_.*"),
        "arm" => bindgen::Builder::default()
            .header(project_dir.join(wrapper).to_str().unwrap())
            .clang_arg(&include_opt)
            .blocklist_function("_Uarm_.*"),
        _ => bindgen::Builder::default()
            .header(project_dir.join(wrapper).to_str().unwrap())
            .clang_arg(&include_opt)
            .blocklist_function("_Ux86_64_.*"),
    };

    let bindings = if env::var_os("CARGO_FEATURE_PTRACE").is_none() {
        bindings
            .blocklist_function("_UPT_.*")
            .blocklist_item("_UPT_.*")
    } else {
        bindings
    };
    let bindings = bindings.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
