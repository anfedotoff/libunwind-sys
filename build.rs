extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut libunwind_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    libunwind_path.push("libunwind");

    let target = env::var("TARGET").unwrap();
    let host  = env::var("HOST").unwrap();
    let split:Vec<&str> = target.split('-').collect();
    let arch = split[0];
    let sys = split[2];
    let abi = split[3];
    //check system libunwind library for linux only
    if sys != "linux" {
        println!("cargo:warning=libunwind supports only Linux");
        return;
    }
    
    //set include directory
    println!("cargo:rustc-link-search=libunwind/src/.libs");

    //choose build
    let (link_lib_arch,link_lib_abi) = match (arch, abi) {
        ("x86_64","gnu") => ("x86_64",""),
        ("x86_64","musl") => ("x86_64","=static"),
        ("i586","gnu")|("i686","gnu") => ("x86",""),
        ("i586","musl")|("i686","musl") => ("x86","=static"),
        ("arm", "gnueabi")|("armv7", "gnueabi") => ("arm",""),
        ("arm", "musleabi")|("armv7", "musleabi") => ("arm","=static"),
        _ => ("","")
    };
    if link_lib_arch.is_empty() {
        println!("cargo:warning=target {} is unsupported",target);
        return;
    }
    //build C libunwind
    Command::new(libunwind_path.join("autogen.sh")).current_dir(&out_dir).status().unwrap();
    //configure. Check if we compile for  x86 target on x86_64 host
    if link_lib_arch == "x86" && host.contains("x86_64") {
        Command::new(libunwind_path.join("configure")).current_dir(&out_dir)
            .arg("CFLAGS=-m32")
            .arg(&format!("--target={}",target))
            .arg(&format!("--host={}",target)).status().unwrap();
    //configure. Check if we compile for  arm target on x86_64 host
    } else  if link_lib_arch == "arm" && host.contains("x86_64") {
        Command::new(libunwind_path.join("configure")).current_dir(&out_dir)
            .arg("CC=arm-linux-gnueabi-gcc")
            .arg(&format!("--target={}",target))
            .arg(&format!("--host={}",target))
            .arg("--disable-tests").status().unwrap();
    }
    else {
        Command::new(libunwind_path.join("configure")).current_dir(&out_dir).arg(&format!("--target={}",target)).status().unwrap();
    }

    let status = Command::new("make").current_dir(&out_dir).status().expect("failed to execute make");
    if !status.success() {
        println!("cargo:warning=build is failed");
        return;
    }
    println!("cargo:rustc-link-lib{}=unwind-coredump",link_lib_abi);
    println!("cargo:rustc-link-lib{}=unwind-{}",link_lib_abi,link_lib_arch);

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("libunwind/include/libunwind-coredump.h")
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

