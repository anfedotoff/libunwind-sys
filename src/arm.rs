//! wrapper for arm  target

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::*;
use libc::c_int;

//Registrers
pub const UNW_TDEP_IP: arm_regnum_t = 14;
pub const UNW_TDEP_SP: arm_regnum_t = 13;
pub const UNW_TDEP_EH: arm_regnum_t = 0;

//functions
extern "C" {
    #[link_name = "_Uarm_create_addr_space"]
    pub fn unw_create_addr_space(
        accessors: *mut unw_accessors_t,
        byteorder: c_int,
    ) -> unw_addr_space_t;

    #[link_name = "_Uarm_destroy_addr_space"]
    pub fn unw_destroy_addr_space(arg1: unw_addr_space_t);

    #[link_name = "_Uarm_get_accessors"]
    pub fn unw_get_accessors(arg1: unw_addr_space_t) -> *mut unw_accessors_t;

    #[link_name = "_Uarm_init_local"]
    pub fn unw_init_local(
        arg1: *mut unw_cursor_t,
        arg2: *mut unw_context_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_init_remote"]
    pub fn unw_init_remote(
        arg1: *mut unw_cursor_t,
        arg2: unw_addr_space_t,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_step"]
    pub fn unw_step(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_resume"]
    pub fn unw_resume(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_get_proc_info"]
    pub fn unw_get_proc_info(
        arg1: *mut unw_cursor_t,
        arg2: *mut unw_proc_info_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_get_reg"]
    pub fn unw_get_reg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut unw_word_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_set_reg"]
    pub fn unw_set_reg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: unw_word_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_get_proc_name"]
    pub fn unw_get_proc_name(
        arg1: *mut unw_cursor_t,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
        arg4: *mut unw_word_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_getcontext"]
    pub fn unw_getcontext(arg1: *mut unw_tdep_context_t) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_strerror"]
    pub fn unw_strerror(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;

    #[link_name = "_Uarm_flush_cache"]
    pub fn unw_flush_cache(arg1: unw_addr_space_t, arg2: unw_word_t, arg3: unw_word_t);

    #[link_name = "_Uarm_regname"]
    pub fn unw_regname(arg1: unw_regnum_t) -> *const ::std::os::raw::c_char;

    #[link_name = "_Uarm_get_proc_info_by_ip"]
    pub fn unw_get_proc_info_by_ip(
        arg1: unw_addr_space_t,
        arg2: unw_word_t,
        arg3: *mut unw_proc_info_t,
        arg4: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_get_fpreg"]
    pub fn unw_get_fpreg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut unw_fpreg_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_set_fpreg"]
    pub fn unw_set_fpreg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: unw_fpreg_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_get_save_loc"]
    pub fn unw_get_save_loc(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut unw_save_loc_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_is_signal_frame"]
    pub fn unw_is_signal_frame(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_handle_signal_frame"]
    pub fn unw_handle_signal_frame(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Uarm_is_fpreg"]
    pub fn unw_is_fpreg(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
