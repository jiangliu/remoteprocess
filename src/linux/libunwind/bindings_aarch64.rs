// automatically generated with:
// bindgen  /usr/include/arm-linux-gnueabihf/libunwind.h --whitelist-type unw_.*

#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __int8_t = ::std::os::raw::c_schar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type size_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigset_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stack_t {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_flags: ::std::os::raw::c_int,
    pub ss_size: size_t,
}
#[test]
fn bindgen_test_layout_stack_t() {
    assert_eq!(
        ::std::mem::size_of::<stack_t>(),
        24usize,
        concat!("Size of: ", stringify!(stack_t))
    );
    assert_eq!(
        ::std::mem::align_of::<stack_t>(),
        8usize,
        concat!("Alignment of ", stringify!(stack_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stack_t>())).ss_sp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(stack_t),
            "::",
            stringify!(ss_sp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stack_t>())).ss_flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(stack_t),
            "::",
            stringify!(ss_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stack_t>())).ss_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(stack_t),
            "::",
            stringify!(ss_size)
        )
    );
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct mcontext_t {
    pub fault_address: ::std::os::raw::c_ulonglong,
    pub regs: [::std::os::raw::c_ulonglong; 31usize],
    pub sp: ::std::os::raw::c_ulonglong,
    pub pc: ::std::os::raw::c_ulonglong,
    pub pstate: ::std::os::raw::c_ulonglong,
    pub __bindgen_padding_0: [u8; 8usize],
    pub __reserved: [::std::os::raw::c_uchar; 4096usize],
}
#[test]
fn bindgen_test_layout_mcontext_t() {
    assert_eq!(
        ::std::mem::size_of::<mcontext_t>(),
        4384usize,
        concat!("Size of: ", stringify!(mcontext_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mcontext_t>(),
        16usize,
        concat!("Alignment of ", stringify!(mcontext_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mcontext_t>())).fault_address as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mcontext_t),
            "::",
            stringify!(fault_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mcontext_t>())).regs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mcontext_t),
            "::",
            stringify!(regs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mcontext_t>())).sp as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(mcontext_t),
            "::",
            stringify!(sp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mcontext_t>())).pc as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(mcontext_t),
            "::",
            stringify!(pc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mcontext_t>())).pstate as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(mcontext_t),
            "::",
            stringify!(pstate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mcontext_t>())).__reserved as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(mcontext_t),
            "::",
            stringify!(__reserved)
        )
    );
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct ucontext_t {
    pub uc_flags: ::std::os::raw::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    pub __bindgen_padding_0: u64,
    pub uc_mcontext: mcontext_t,
}
#[test]
fn bindgen_test_layout_ucontext_t() {
    assert_eq!(
        ::std::mem::size_of::<ucontext_t>(),
        4560usize,
        concat!("Size of: ", stringify!(ucontext_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ucontext_t>(),
        16usize,
        concat!("Alignment of ", stringify!(ucontext_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ucontext_t>())).uc_flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ucontext_t),
            "::",
            stringify!(uc_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ucontext_t>())).uc_link as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ucontext_t),
            "::",
            stringify!(uc_link)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ucontext_t>())).uc_stack as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ucontext_t),
            "::",
            stringify!(uc_stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ucontext_t>())).uc_sigmask as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ucontext_t),
            "::",
            stringify!(uc_sigmask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ucontext_t>())).uc_mcontext as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(ucontext_t),
            "::",
            stringify!(uc_mcontext)
        )
    );
}
pub type unw_word_t = u64;
pub type unw_sword_t = i64;
pub type unw_tdep_fpreg_t = u128;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_tdep_proc_info_t {}
#[test]
fn bindgen_test_layout_unw_tdep_proc_info_t() {
    assert_eq!(
        ::std::mem::size_of::<unw_tdep_proc_info_t>(),
        0usize,
        concat!("Size of: ", stringify!(unw_tdep_proc_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_tdep_proc_info_t>(),
        1usize,
        concat!("Alignment of ", stringify!(unw_tdep_proc_info_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_tdep_save_loc {}
#[test]
fn bindgen_test_layout_unw_tdep_save_loc() {
    assert_eq!(
        ::std::mem::size_of::<unw_tdep_save_loc>(),
        0usize,
        concat!("Size of: ", stringify!(unw_tdep_save_loc))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_tdep_save_loc>(),
        1usize,
        concat!("Alignment of ", stringify!(unw_tdep_save_loc))
    );
}
pub type unw_tdep_save_loc_t = unw_tdep_save_loc;
pub type unw_tdep_context_t = ucontext_t;
pub const unw_error_t_UNW_ESUCCESS: unw_error_t = 0;
pub const unw_error_t_UNW_EUNSPEC: unw_error_t = 1;
pub const unw_error_t_UNW_ENOMEM: unw_error_t = 2;
pub const unw_error_t_UNW_EBADREG: unw_error_t = 3;
pub const unw_error_t_UNW_EREADONLYREG: unw_error_t = 4;
pub const unw_error_t_UNW_ESTOPUNWIND: unw_error_t = 5;
pub const unw_error_t_UNW_EINVALIDIP: unw_error_t = 6;
pub const unw_error_t_UNW_EBADFRAME: unw_error_t = 7;
pub const unw_error_t_UNW_EINVAL: unw_error_t = 8;
pub const unw_error_t_UNW_EBADVERSION: unw_error_t = 9;
pub const unw_error_t_UNW_ENOINFO: unw_error_t = 10;
pub type unw_error_t = ::std::os::raw::c_uint;
pub const unw_frame_regnum_t_UNW_REG_IP: unw_frame_regnum_t = 30;
pub const unw_frame_regnum_t_UNW_REG_SP: unw_frame_regnum_t = 31;
pub const unw_frame_regnum_t_UNW_REG_EH: unw_frame_regnum_t = 0;
pub const unw_frame_regnum_t_UNW_REG_LAST: unw_frame_regnum_t = 97;
pub type unw_frame_regnum_t = ::std::os::raw::c_uint;
pub const unw_caching_policy_t_UNW_CACHE_NONE: unw_caching_policy_t = 0;
pub const unw_caching_policy_t_UNW_CACHE_GLOBAL: unw_caching_policy_t = 1;
pub const unw_caching_policy_t_UNW_CACHE_PER_THREAD: unw_caching_policy_t = 2;
pub type unw_caching_policy_t = ::std::os::raw::c_uint;
pub const unw_init_local2_flags_t_UNW_INIT_SIGNAL_FRAME: unw_init_local2_flags_t = 1;
pub type unw_init_local2_flags_t = ::std::os::raw::c_uint;
pub type unw_regnum_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_cursor {
    pub opaque: [unw_word_t; 512usize],
}
#[test]
fn bindgen_test_layout_unw_cursor() {
    assert_eq!(
        ::std::mem::size_of::<unw_cursor>(),
        4096usize,
        concat!("Size of: ", stringify!(unw_cursor))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_cursor>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_cursor))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_cursor>())).opaque as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_cursor),
            "::",
            stringify!(opaque)
        )
    );
}
pub type unw_cursor_t = unw_cursor;
pub type unw_context_t = unw_tdep_context_t;
pub type unw_fpreg_t = unw_tdep_fpreg_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_addr_space {
    _unused: [u8; 0],
}
pub type unw_addr_space_t = *mut unw_addr_space;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_proc_info {
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub lsda: unw_word_t,
    pub handler: unw_word_t,
    pub gp: unw_word_t,
    pub flags: unw_word_t,
    pub format: ::std::os::raw::c_int,
    pub unwind_info_size: ::std::os::raw::c_int,
    pub unwind_info: *mut ::std::os::raw::c_void,
    pub extra: unw_tdep_proc_info_t,
}
#[test]
fn bindgen_test_layout_unw_proc_info() {
    assert_eq!(
        ::std::mem::size_of::<unw_proc_info>(),
        64usize,
        concat!("Size of: ", stringify!(unw_proc_info))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_proc_info>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_proc_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).start_ip as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(start_ip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).end_ip as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(end_ip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).lsda as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(lsda)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).handler as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).gp as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(gp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).flags as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).format as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).unwind_info_size as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(unwind_info_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).unwind_info as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(unwind_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_proc_info>())).extra as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_proc_info),
            "::",
            stringify!(extra)
        )
    );
}
pub type unw_proc_info_t = unw_proc_info;
pub type unw_reg_states_callback = ::std::option::Option<
    unsafe extern "C" fn(
        token: *mut ::std::os::raw::c_void,
        reg_states_data: *mut ::std::os::raw::c_void,
        reg_states_data_size: size_t,
        start_ip: unw_word_t,
        end_ip: unw_word_t,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_accessors {
    pub find_proc_info: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: unw_word_t,
            arg3: *mut unw_proc_info_t,
            arg4: ::std::os::raw::c_int,
            arg5: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub put_unwind_info: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: *mut unw_proc_info_t,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
    pub get_dyn_info_list_addr: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: *mut unw_word_t,
            arg3: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub access_mem: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: unw_word_t,
            arg3: *mut unw_word_t,
            arg4: ::std::os::raw::c_int,
            arg5: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub access_reg: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: unw_regnum_t,
            arg3: *mut unw_word_t,
            arg4: ::std::os::raw::c_int,
            arg5: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub access_fpreg: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: unw_regnum_t,
            arg3: *mut unw_fpreg_t,
            arg4: ::std::os::raw::c_int,
            arg5: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub resume: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: *mut unw_cursor_t,
            arg3: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_proc_name: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: unw_addr_space_t,
            arg2: unw_word_t,
            arg3: *mut ::std::os::raw::c_char,
            arg4: size_t,
            arg5: *mut unw_word_t,
            arg6: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
#[test]
fn bindgen_test_layout_unw_accessors() {
    assert_eq!(
        ::std::mem::size_of::<unw_accessors>(),
        64usize,
        concat!("Size of: ", stringify!(unw_accessors))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_accessors>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_accessors))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).find_proc_info as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(find_proc_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).put_unwind_info as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(put_unwind_info)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<unw_accessors>())).get_dyn_info_list_addr as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(get_dyn_info_list_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).access_mem as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(access_mem)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).access_reg as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(access_reg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).access_fpreg as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(access_fpreg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).resume as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(resume)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_accessors>())).get_proc_name as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_accessors),
            "::",
            stringify!(get_proc_name)
        )
    );
}
pub type unw_accessors_t = unw_accessors;
pub const unw_save_loc_type_UNW_SLT_NONE: unw_save_loc_type = 0;
pub const unw_save_loc_type_UNW_SLT_MEMORY: unw_save_loc_type = 1;
pub const unw_save_loc_type_UNW_SLT_REG: unw_save_loc_type = 2;
pub type unw_save_loc_type = ::std::os::raw::c_uint;
pub use self::unw_save_loc_type as unw_save_loc_type_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_save_loc {
    pub type_: unw_save_loc_type_t,
    pub u: unw_save_loc__bindgen_ty_1,
    pub extra: unw_tdep_save_loc_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union unw_save_loc__bindgen_ty_1 {
    pub addr: unw_word_t,
    pub regnum: unw_regnum_t,
}
#[test]
fn bindgen_test_layout_unw_save_loc__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<unw_save_loc__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(unw_save_loc__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_save_loc__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_save_loc__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_save_loc__bindgen_ty_1>())).addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_save_loc__bindgen_ty_1),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<unw_save_loc__bindgen_ty_1>())).regnum as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_save_loc__bindgen_ty_1),
            "::",
            stringify!(regnum)
        )
    );
}
#[test]
fn bindgen_test_layout_unw_save_loc() {
    assert_eq!(
        ::std::mem::size_of::<unw_save_loc>(),
        16usize,
        concat!("Size of: ", stringify!(unw_save_loc))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_save_loc>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_save_loc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_save_loc>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_save_loc),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_save_loc>())).u as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_save_loc),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_save_loc>())).extra as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_save_loc),
            "::",
            stringify!(extra)
        )
    );
}
pub type unw_save_loc_t = unw_save_loc;
pub const unw_dyn_operation_t_UNW_DYN_STOP: unw_dyn_operation_t = 0;
pub const unw_dyn_operation_t_UNW_DYN_SAVE_REG: unw_dyn_operation_t = 1;
pub const unw_dyn_operation_t_UNW_DYN_SPILL_FP_REL: unw_dyn_operation_t = 2;
pub const unw_dyn_operation_t_UNW_DYN_SPILL_SP_REL: unw_dyn_operation_t = 3;
pub const unw_dyn_operation_t_UNW_DYN_ADD: unw_dyn_operation_t = 4;
pub const unw_dyn_operation_t_UNW_DYN_POP_FRAMES: unw_dyn_operation_t = 5;
pub const unw_dyn_operation_t_UNW_DYN_LABEL_STATE: unw_dyn_operation_t = 6;
pub const unw_dyn_operation_t_UNW_DYN_COPY_STATE: unw_dyn_operation_t = 7;
pub const unw_dyn_operation_t_UNW_DYN_ALIAS: unw_dyn_operation_t = 8;
pub type unw_dyn_operation_t = ::std::os::raw::c_uint;
pub const unw_dyn_info_format_t_UNW_INFO_FORMAT_DYNAMIC: unw_dyn_info_format_t = 0;
pub const unw_dyn_info_format_t_UNW_INFO_FORMAT_TABLE: unw_dyn_info_format_t = 1;
pub const unw_dyn_info_format_t_UNW_INFO_FORMAT_REMOTE_TABLE: unw_dyn_info_format_t = 2;
pub const unw_dyn_info_format_t_UNW_INFO_FORMAT_ARM_EXIDX: unw_dyn_info_format_t = 3;
pub const unw_dyn_info_format_t_UNW_INFO_FORMAT_IP_OFFSET: unw_dyn_info_format_t = 4;
pub type unw_dyn_info_format_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_dyn_op {
    pub tag: i8,
    pub qp: i8,
    pub reg: i16,
    pub when: i32,
    pub val: unw_word_t,
}
#[test]
fn bindgen_test_layout_unw_dyn_op() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_op>(),
        16usize,
        concat!("Size of: ", stringify!(unw_dyn_op))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_op>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_op))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_op>())).tag as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_op),
            "::",
            stringify!(tag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_op>())).qp as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_op),
            "::",
            stringify!(qp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_op>())).reg as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_op),
            "::",
            stringify!(reg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_op>())).when as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_op),
            "::",
            stringify!(when)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_op>())).val as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_op),
            "::",
            stringify!(val)
        )
    );
}
pub type unw_dyn_op_t = unw_dyn_op;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_dyn_region_info {
    pub next: *mut unw_dyn_region_info,
    pub insn_count: i32,
    pub op_count: u32,
    pub op: [unw_dyn_op_t; 1usize],
}
#[test]
fn bindgen_test_layout_unw_dyn_region_info() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_region_info>(),
        32usize,
        concat!("Size of: ", stringify!(unw_dyn_region_info))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_region_info>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_region_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_region_info>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_region_info),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_region_info>())).insn_count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_region_info),
            "::",
            stringify!(insn_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_region_info>())).op_count as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_region_info),
            "::",
            stringify!(op_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_region_info>())).op as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_region_info),
            "::",
            stringify!(op)
        )
    );
}
pub type unw_dyn_region_info_t = unw_dyn_region_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_dyn_proc_info {
    pub name_ptr: unw_word_t,
    pub handler: unw_word_t,
    pub flags: u32,
    pub pad0: i32,
    pub regions: *mut unw_dyn_region_info_t,
}
#[test]
fn bindgen_test_layout_unw_dyn_proc_info() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_proc_info>(),
        32usize,
        concat!("Size of: ", stringify!(unw_dyn_proc_info))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_proc_info>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_proc_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_proc_info>())).name_ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_proc_info),
            "::",
            stringify!(name_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_proc_info>())).handler as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_proc_info),
            "::",
            stringify!(handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_proc_info>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_proc_info),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_proc_info>())).pad0 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_proc_info),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_proc_info>())).regions as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_proc_info),
            "::",
            stringify!(regions)
        )
    );
}
pub type unw_dyn_proc_info_t = unw_dyn_proc_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_dyn_table_info {
    pub name_ptr: unw_word_t,
    pub segbase: unw_word_t,
    pub table_len: unw_word_t,
    pub table_data: *mut unw_word_t,
}
#[test]
fn bindgen_test_layout_unw_dyn_table_info() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_table_info>(),
        32usize,
        concat!("Size of: ", stringify!(unw_dyn_table_info))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_table_info>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_table_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_table_info>())).name_ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_table_info),
            "::",
            stringify!(name_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_table_info>())).segbase as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_table_info),
            "::",
            stringify!(segbase)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_table_info>())).table_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_table_info),
            "::",
            stringify!(table_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_table_info>())).table_data as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_table_info),
            "::",
            stringify!(table_data)
        )
    );
}
pub type unw_dyn_table_info_t = unw_dyn_table_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_dyn_remote_table_info {
    pub name_ptr: unw_word_t,
    pub segbase: unw_word_t,
    pub table_len: unw_word_t,
    pub table_data: unw_word_t,
}
#[test]
fn bindgen_test_layout_unw_dyn_remote_table_info() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_remote_table_info>(),
        32usize,
        concat!("Size of: ", stringify!(unw_dyn_remote_table_info))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_remote_table_info>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_remote_table_info))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<unw_dyn_remote_table_info>())).name_ptr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_remote_table_info),
            "::",
            stringify!(name_ptr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<unw_dyn_remote_table_info>())).segbase as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_remote_table_info),
            "::",
            stringify!(segbase)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<unw_dyn_remote_table_info>())).table_len as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_remote_table_info),
            "::",
            stringify!(table_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<unw_dyn_remote_table_info>())).table_data as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_remote_table_info),
            "::",
            stringify!(table_data)
        )
    );
}
pub type unw_dyn_remote_table_info_t = unw_dyn_remote_table_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_dyn_info {
    pub next: *mut unw_dyn_info,
    pub prev: *mut unw_dyn_info,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub gp: unw_word_t,
    pub format: i32,
    pub pad: i32,
    pub u: unw_dyn_info__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union unw_dyn_info__bindgen_ty_1 {
    pub pi: unw_dyn_proc_info_t,
    pub ti: unw_dyn_table_info_t,
    pub rti: unw_dyn_remote_table_info_t,
}
#[test]
fn bindgen_test_layout_unw_dyn_info__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_info__bindgen_ty_1>(),
        32usize,
        concat!("Size of: ", stringify!(unw_dyn_info__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_info__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_info__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info__bindgen_ty_1>())).pi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info__bindgen_ty_1),
            "::",
            stringify!(pi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info__bindgen_ty_1>())).ti as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info__bindgen_ty_1),
            "::",
            stringify!(ti)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info__bindgen_ty_1>())).rti as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info__bindgen_ty_1),
            "::",
            stringify!(rti)
        )
    );
}
#[test]
fn bindgen_test_layout_unw_dyn_info() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_info>(),
        80usize,
        concat!("Size of: ", stringify!(unw_dyn_info))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_info>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).prev as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).start_ip as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(start_ip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).end_ip as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(end_ip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).gp as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(gp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).format as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).pad as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info>())).u as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info),
            "::",
            stringify!(u)
        )
    );
}
pub type unw_dyn_info_t = unw_dyn_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unw_dyn_info_list {
    pub version: u32,
    pub generation: u32,
    pub first: *mut unw_dyn_info_t,
}
#[test]
fn bindgen_test_layout_unw_dyn_info_list() {
    assert_eq!(
        ::std::mem::size_of::<unw_dyn_info_list>(),
        16usize,
        concat!("Size of: ", stringify!(unw_dyn_info_list))
    );
    assert_eq!(
        ::std::mem::align_of::<unw_dyn_info_list>(),
        8usize,
        concat!("Alignment of ", stringify!(unw_dyn_info_list))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info_list>())).version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info_list),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info_list>())).generation as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info_list),
            "::",
            stringify!(generation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<unw_dyn_info_list>())).first as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(unw_dyn_info_list),
            "::",
            stringify!(first)
        )
    );
}
pub type unw_dyn_info_list_t = unw_dyn_info_list;
