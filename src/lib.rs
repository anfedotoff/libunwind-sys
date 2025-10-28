//! Low-level bindings for the [libunwind] library.
//!
//! Please see the libunwind  [C API documentation] for function descriptions.
//!
//! [libunwind]: http://www.nongnu.org/libunwind/
//! [C API documentation]: https://www.nongnu.org/libunwind/docs.html

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use crate::native::*;

#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
#[cfg_attr(target_arch = "x86", path = "x86.rs")]
#[cfg_attr(target_arch = "arm", path = "arm.rs")]
mod native;

#[cfg(test)]
mod tests {
    extern crate libc;

    use crate::*;
    use libc::c_char;
    use std::ffi::{CStr, CString};
    use std::mem::MaybeUninit;
    use std::path::PathBuf;

    unsafe fn unwind_core_dump(core_file_name: &str) -> String {
        let mut core_path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        core_path_buf.push(format!("data/{}", core_file_name));

        let core_path = CString::new(core_path_buf.to_str().unwrap()).unwrap();

        let asp = unw_create_addr_space(&raw mut _UCD_accessors, 0);
        let ui: *mut UCD_info = _UCD_create(core_path.as_ptr());
        let mut c = MaybeUninit::uninit();
        let _ret = unw_init_remote(c.as_mut_ptr(), asp, ui as *mut libc::c_void);

        let mut ip: unw_word_t = 0;
        let mut sp: unw_word_t = 0;
        let mut val: unw_word_t = 0;
        let mut backtrace = String::new();

        loop {
            unw_get_reg(c.as_mut_ptr(), UNW_TDEP_IP as ::std::os::raw::c_int, &mut ip);
            unw_get_reg(c.as_mut_ptr(), UNW_TDEP_SP as ::std::os::raw::c_int, &mut sp);
            let ret = _UCD_access_mem(asp, sp, &mut val, 0, ui as *mut libc::c_void);
            if ret < 0 {
                assert!(false);
            }
            let mut off = MaybeUninit::uninit();
            let mut name_vec: Vec<c_char> = vec![0; 64];
            unw_get_proc_name(
                c.as_mut_ptr(),
                name_vec.as_mut_ptr(),
                64,
                off.as_mut_ptr(),
            );
            let name = CStr::from_ptr(name_vec.as_mut_ptr());
            backtrace.push_str(&format!("0x{:x} in {:?} ()\n", ip, name.to_str().unwrap()));
            let ret = unw_step(c.as_mut_ptr());
            if ret <= 0 {
                break;
            }
        }

        backtrace
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_core_unwind() {
        unsafe {
            let backtrace = unwind_core_dump("core.test_callstack");
            assert!(backtrace.contains("main"));
            assert!(backtrace.contains("first"));
            assert!(backtrace.contains("second"));
            assert!(backtrace.contains("third"));
        }
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_core_unwind_heap_error() {
        unsafe {
            let backtrace = unwind_core_dump("core.test_heapError");
            assert!(backtrace.contains("main"));
            assert!(backtrace.contains("cfree"));
        }
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_core_unwind_canary() {
        unsafe {
            let backtrace = unwind_core_dump("core.test_canary");
            assert!(backtrace.contains("main"));
            assert!(backtrace.contains("fortify_fail"));
        }
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_local_unwind() {
        unsafe {
            let mut c  = MaybeUninit::uninit();
            let mut uc  = MaybeUninit::uninit();
            let mut ip: unw_word_t = 0;
            let _ret = unw_getcontext(uc.as_mut_ptr());
            let _ret = unw_init_local(c.as_mut_ptr(),uc.as_mut_ptr());
            let mut backtrace = String::new();
            loop {
                unw_get_reg(c.as_mut_ptr(), UNW_TDEP_IP as ::std::os::raw::c_int, &mut ip);
                let mut off  = MaybeUninit::uninit();
                let mut name_vec:Vec<c_char> = vec![0;64];
                unw_get_proc_name(c.as_mut_ptr(), name_vec.as_mut_ptr(),64, off.as_mut_ptr());
                let name = CStr::from_ptr(name_vec.as_mut_ptr());
                backtrace.push_str(&format!("0x{:x} in {:?} ()\n", ip, name.to_str().unwrap()));
                let ret = unw_step(c.as_mut_ptr());
                if ret <= 0 {
                    break;
                }
            }
            assert!(backtrace.contains("test_local_unwind"));
            assert!(backtrace.contains("start_thread") || backtrace.contains("start"));
        }
    }

    #[test]
    #[cfg(all(feature = "ptrace", target_arch = "x86_64"))]
    fn test_remote_unwind() {
        unsafe {
            let mut c  = MaybeUninit::uninit();
            let mut ip: unw_word_t = 0;
            let asp = unw_create_addr_space(&mut _UPT_accessors ,0);
            //spawn child proccess
            let mut test_callstack_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            test_callstack_path_buf.push("data/test_callstack_remote");
            let mut child = Command::new(test_callstack_path_buf.to_str().unwrap())
                .spawn()
                .expect("failed to execute child");
             thread::sleep(Duration::from_millis(10));
            let ret = libc::ptrace(
                libc::PTRACE_ATTACH,
                child.id() as libc::pid_t,
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
            );
            if ret != 0 {
                panic!("{}", io::Error::last_os_error());
            }

            loop {
                let mut status = 0;
                let ret = libc::waitpid(child.id() as libc::pid_t, &mut status, 0);
                if ret < 0 {
                    panic!("{}", io::Error::last_os_error());
                }
                if libc::WIFSTOPPED(status) {
                    break;
                }
            }

            let ui: *mut ::std::os::raw::c_void = _UPT_create(child.id() as i32);
            let mut backtrace = String::new();

            let _ret = unw_init_remote(c.as_mut_ptr(),asp,ui as * mut libc::c_void );
            loop {
                unw_get_reg(c.as_mut_ptr(), UNW_TDEP_IP as ::std::os::raw::c_int, &mut ip);
                let mut off  = MaybeUninit::uninit();
                let mut name_vec:Vec<c_char> = vec![0;64];
                unw_get_proc_name(c.as_mut_ptr(), name_vec.as_mut_ptr(),64, off.as_mut_ptr());
                let name = CStr::from_ptr(name_vec.as_mut_ptr());
                backtrace.push_str(&format!("0x{:x} in {:?} ()\n", ip, name.to_str().unwrap()));
                let ret =  unw_step(c.as_mut_ptr());
                if ret <= 0 {
                    break;
                }
            }
            assert!(backtrace.contains("main"));
            assert!(backtrace.contains("first"));
            assert!(backtrace.contains("second"));
            assert!(backtrace.contains("third"));
            _UPT_destroy(ui);
            unw_destroy_addr_space(asp);
            child.kill().unwrap();
        }
    }
}
