extern crate bindgen;
extern crate fs_extra;
extern crate autotools;

use autotools::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut libunwind_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    libunwind_path.push("libunwind");
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _e = fs_extra::dir::create(libunwind_path.clone(),true);
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::dir::copy(project_dir.join("libunwind"), out_dir.clone(), &options).unwrap();
    
    let target = env::var("TARGET").unwrap();
    let host  = env::var("HOST").unwrap();
    
    //choose build
    let link_lib_arch = match target.as_str() {
        "x86_64-unknown-linux-gnu" => "x86_64",
        "i686-unknown-linux-gnu"|"i586-unknown-linux-gnu"  => "x86",
        "arm-unknown-linux-gnueabihf"|"armv7-unknown-linux-gnueabihf" => "arm",
        _ => ""
    };
    if link_lib_arch.is_empty() {
        println!("cargo:warning=target {} is unsupported",target);
        return;
    }
    //build C libunwind
    let _autogen = Command::new("sh").current_dir(&libunwind_path)
                                     .arg("-c")
                                     .arg(format!("autoreconf --force --install --verbose {}",&libunwind_path.to_str().unwrap()))
                                     .output()
                                     .expect("failed to run autoreconf, do you have the autotools installed?");
    //configure. Check if we compile for  x86 target on x86_64 host
    let dst = if link_lib_arch == "x86" && host.contains("x86_64") {
        Config::new(&libunwind_path)
            .cflag("-m32")
            .target(&target)
            .host(&target)
            .disable("documentation", None)
            .disable("tests", None)
            .enable_shared().build()

    //configure. Check if we compile for  arm target on x86_64 host
    } else  if link_lib_arch == "arm" && host.contains("x86_64") {
        Config::new(&libunwind_path)
            .env("CC","arm-linux-gnueabihf-gcc")
            .target(&target)
            .host(&target)
            .disable("documentation", None)
            .disable("tests", None)
            .enable_shared().build()
    }
    else {
       Config::new(&libunwind_path).disable("documentation", None).disable("tests", None).enable_shared().build()
    };
    
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=unwind-coredump");
    println!("cargo:rustc-link-lib=unwind-{}",link_lib_arch);
    println!("cargo:rustc-link-lib=unwind");
    println!("cargo:rustc-link-lib=unwind-ptrace");
    //choose header
    let wrapper =  if link_lib_arch == "arm" && host.contains("x86_64") {
        "wrapper-arm.h"
    } else {
        "wrapper.h"
    };
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(project_dir.join(wrapper).to_str().unwrap())
        //include directory
        .clang_arg("-Ilibunwind/include")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

