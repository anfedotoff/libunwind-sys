//! wrapper for x86_64 target

use crate::*;
use libc::c_int;

//Registrers
pub const UNW_TDEP_IP: u32 = x86_64_regnum_t_UNW_X86_64_RIP;
pub const UNW_TDEP_SP: u32 = x86_64_regnum_t_UNW_X86_64_RSP;
pub const UNW_TDEP_BP: u32 = x86_64_regnum_t_UNW_X86_64_RBP;
pub const UNW_TDEP_EH: u32 = x86_64_regnum_t_UNW_X86_64_RAX;

//functions
extern "C" {
    #[link_name = "_Ux86_64_create_addr_space"]
    pub fn unw_create_addr_space(
        accessors: *mut unw_accessors_t,
        byteorder: c_int,
    ) -> unw_addr_space_t;

    #[link_name = "_Ux86_64_destroy_addr_space"]
    pub fn unw_destroy_addr_space(arg1: unw_addr_space_t);

    #[link_name = "_Ux86_64_get_accessors"]
    pub fn unw_get_accessors(arg1: unw_addr_space_t) -> *mut unw_accessors_t;

    #[link_name = "_Ux86_64_init_local"]
    pub fn unw_init_local(
        arg1: *mut unw_cursor_t,
        arg2: *mut unw_context_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_init_remote"]
    pub fn unw_init_remote(
        arg1: *mut unw_cursor_t,
        arg2: unw_addr_space_t,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_step"]
    pub fn unw_step(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_resume"]
    pub fn unw_resume(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_get_proc_info"]
    pub fn unw_get_proc_info(
        arg1: *mut unw_cursor_t,
        arg2: *mut unw_proc_info_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_get_reg"]
    pub fn unw_get_reg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut unw_word_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_set_reg"]
    pub fn unw_set_reg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: unw_word_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_get_proc_name"]
    pub fn unw_get_proc_name(
        arg1: *mut unw_cursor_t,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
        arg4: *mut unw_word_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_getcontext"]
    pub fn unw_getcontext(arg1: *mut unw_tdep_context_t) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_strerror"]
    pub fn unw_strerror(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;

    #[link_name = "_Ux86_64_flush_cache"]
    pub fn unw_flush_cache(arg1: unw_addr_space_t, arg2: unw_word_t, arg3: unw_word_t);

    #[link_name = "_Ux86_64_set_caching_policy"]
    pub fn unw_set_caching_policy(
        arg1: unw_addr_space_t,
        arg2: unw_caching_policy_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_get_fpreg"]
    pub fn unw_get_fpreg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut unw_fpreg_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_regname"]
    pub fn unw_regname(arg1: unw_regnum_t) -> *const ::std::os::raw::c_char;

    #[link_name = "_Ux86_64_get_proc_info_by_ip"]
    pub fn unw_get_proc_info_by_ip(
        arg1: unw_addr_space_t,
        arg2: unw_word_t,
        arg3: *mut unw_proc_info_t,
        arg4: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_set_fpreg"]
    pub fn unw_set_fpreg(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: unw_fpreg_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_get_save_loc"]
    pub fn unw_get_save_loc(
        arg1: *mut unw_cursor_t,
        arg2: ::std::os::raw::c_int,
        arg3: *mut unw_save_loc_t,
    ) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_is_signal_frame"]
    pub fn unw_is_signal_frame(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_handle_signal_frame"]
    pub fn unw_handle_signal_frame(arg1: *mut unw_cursor_t) -> ::std::os::raw::c_int;

    #[link_name = "_Ux86_64_is_fpreg"]
    pub fn unw_is_fpreg(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
