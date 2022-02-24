extern crate autotools;
extern crate bindgen;
extern crate fs_extra;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut libunwind_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    libunwind_path.push("libunwind");
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _e = fs_extra::dir::create(libunwind_path.clone(), true);
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::dir::copy(project_dir.join("libunwind"), out_dir.clone(), &options).unwrap();

    let target = env::var("TARGET").unwrap();

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

    // Build native C library only for static link.
    #[cfg(feature = "static")]
    {
        use autotools::Config;

        // Build libunwind.
        // Configure. Check if we compile for  x86 target on x86_64 host.
        let mut dst = Config::new(&libunwind_path);

        cfg_if::cfg_if! {
            if #[cfg(feature = "ptrace")] {
                dst.enable("ptrace", None);
            } else {
                dst.disable("ptrace", None);
            }
        }

        dst.disable("documentation", None)
            .disable("tests", None)
            .disable_shared()
            .enable_static();

        let dst = dst.build();
        println!("cargo:rustc-link-search={}/lib", dst.display());
    }

    println!("cargo:rustc-link-lib=unwind-{}", link_lib_arch);
    println!("cargo:rustc-link-lib=unwind");
    println!("cargo:rustc-link-lib=unwind-coredump");
    #[cfg(feature = "ptrace")]
    {
        println!("cargo:rustc-link-lib=unwind-ptrace");
    }

    let bindings = bindgen::Builder::default();
    let bindings = match link_lib_arch {
        "x86" => bindings.blocklist_function("_Ux86_.*"),
        "arm" => bindings.blocklist_function("_Uarm_.*"),
        _ => bindings.blocklist_function("_Ux86_64_.*"),
    };
    let bindings = bindings.header("libunwind/include/libunwind.h");
    let bindings = bindings.header("libunwind/include/libunwind-coredump.h");

    #[cfg(feature = "ptrace")]
    let bindings = {
       bindings.header("libunwind/include/libunwind-ptrace.h")
    };

    let bindings = bindings.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
