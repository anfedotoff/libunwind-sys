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
#[cfg(test)]
mod tests {
    extern crate libc;

    use super::*;
    use libc::c_char;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::mem::MaybeUninit; 
    use std::path::PathBuf;

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_core_unwind() {
        unsafe {
            let mut libc_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            libc_path_buf.push("data/libc-2.23.so");
            let mut test_callstack_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            test_callstack_path_buf.push("data/test_callstack");
            let mut core_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            core_path_buf.push("data/core.test_callstack");

            let test_callstack_start:u64 = 0x400000;
            let libc_start:u64 = 0x00007f9ac7468000;
            let test_callstack_path = CString::new(test_callstack_path_buf.to_str().unwrap()).unwrap();
            let libc_path = CString::new(libc_path_buf.to_str().unwrap()).unwrap();
            let core_path = CString::new(core_path_buf.to_str().unwrap()).unwrap();

            println!("{:#?}\n{:#?}\n{:#?}",test_callstack_path,libc_path, core_path);
            let asp = _Ux86_64_create_addr_space(&mut _UCD_accessors ,0);
            let ui: * mut UCD_info = _UCD_create(core_path.as_ptr());
            let mut c  = MaybeUninit::uninit();
            let mut ret = _Ux86_64_init_remote(c.as_mut_ptr(),asp,ui as * mut libc::c_void );
            _UCD_add_backing_file_at_vaddr(ui, test_callstack_start, test_callstack_path.as_ptr());
            
            _UCD_add_backing_file_at_vaddr(ui, libc_start, libc_path.as_ptr());
           let mut ip: unw_word_t = 0;
           loop {
           _Ux86_64_get_reg(c.as_mut_ptr(), unw_frame_regnum_t_UNW_REG_IP as ::std::os::raw::c_int, &mut ip);
              let mut off  = MaybeUninit::uninit();
              let mut name_vec:Vec<c_char> = vec![0;64];
              _Ux86_64_get_proc_name(c.as_mut_ptr(), name_vec.as_mut_ptr(),64, off.as_mut_ptr());
              let name = CStr::from_ptr(name_vec.as_mut_ptr());
              println!("0x{:x} in {:?} ()", ip, name.to_str().unwrap());
              ret = _Ux86_64_step(c.as_mut_ptr());
              if ret <= 0 {
                  break;
              }
           }
        }
    }
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_core_unwind_heap_error() {
        unsafe {
            let mut libc_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            libc_path_buf.push("data/libc-2.23.so");
            let mut test_heap_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            test_heap_path_buf.push("data/test_heapError");
            let mut core_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            core_path_buf.push("data/core.test_heapError");

            let test_heap_start:u64 = 0x000055b7b218c000;
            let libc_start:u64 = 0x00007f90e058b000;
            let test_heap_path = CString::new(test_heap_path_buf.to_str().unwrap()).unwrap();
            let libc_path = CString::new(libc_path_buf.to_str().unwrap()).unwrap();
            let core_path = CString::new(core_path_buf.to_str().unwrap()).unwrap();
            let asp = _Ux86_64_create_addr_space(&mut _UCD_accessors ,0);
            let ui: * mut UCD_info = _UCD_create(core_path.as_ptr());
            let mut c  = MaybeUninit::uninit();
            let mut ret = _Ux86_64_init_remote(c.as_mut_ptr(),asp,ui as * mut libc::c_void );
            _UCD_add_backing_file_at_vaddr(ui, test_heap_start, test_heap_path.as_ptr());
            
            _UCD_add_backing_file_at_vaddr(ui, libc_start, libc_path.as_ptr());
           let mut ip: unw_word_t = 0;
           loop {
           _Ux86_64_get_reg(c.as_mut_ptr(), unw_frame_regnum_t_UNW_REG_IP as ::std::os::raw::c_int, &mut ip);
              let mut off  = MaybeUninit::uninit();
              let mut name_vec:Vec<c_char> = vec![0;64];
              _Ux86_64_get_proc_name(c.as_mut_ptr(), name_vec.as_mut_ptr(),64, off.as_mut_ptr());
              let name = CStr::from_ptr(name_vec.as_mut_ptr());
              println!("0x{:x} in {:?} ()", ip, name.to_str().unwrap());
              ret = _Ux86_64_step(c.as_mut_ptr());
              if ret <= 0 {
                  break;
              }
           }
        }
    }
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_core_unwind_canary() {
        unsafe {
            let mut libc_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            libc_path_buf.push("data/libc-2.23.so");
            let mut test_canary_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            test_canary_path_buf.push("data/test_canary");
            let mut core_path_buf  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            core_path_buf.push("data/core.test_canary");

            let test_canary_start:u64 = 0x0000558672376000;
            let libc_start:u64 = 0x00007fc14b336000;
            let test_canary_path = CString::new(test_canary_path_buf.to_str().unwrap()).unwrap();
            let libc_path = CString::new(libc_path_buf.to_str().unwrap()).unwrap();
            let core_path = CString::new(core_path_buf.to_str().unwrap()).unwrap();
            let asp = _Ux86_64_create_addr_space(&mut _UCD_accessors ,0);
            let ui: * mut UCD_info = _UCD_create(core_path.as_ptr());
            let mut c  = MaybeUninit::uninit();
            let mut ret = _Ux86_64_init_remote(c.as_mut_ptr(),asp,ui as * mut libc::c_void );
            _UCD_add_backing_file_at_vaddr(ui, test_canary_start, test_canary_path.as_ptr());
            
            _UCD_add_backing_file_at_vaddr(ui, libc_start, libc_path.as_ptr());
           let mut ip: unw_word_t = 0;
           loop {
           _Ux86_64_get_reg(c.as_mut_ptr(), unw_frame_regnum_t_UNW_REG_IP as ::std::os::raw::c_int, &mut ip);
              let mut off  = MaybeUninit::uninit();
              let mut name_vec:Vec<c_char> = vec![0;64];
              _Ux86_64_get_proc_name(c.as_mut_ptr(), name_vec.as_mut_ptr(),64, off.as_mut_ptr());
              let name = CStr::from_ptr(name_vec.as_mut_ptr());
              println!("0x{:x} in {:?} ()", ip, name.to_str().unwrap());
              ret = _Ux86_64_step(c.as_mut_ptr());
              if ret <= 0 {
                  break;
              }
           }
        }
    }
}
