use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn tpde_initialize_session();
    fn tpde_emit_phi_node(
        dest: *const c_char,
        blocks: *const *const c_char,
        values: *const c_int,
        count: c_int,
    );
}

pub fn init() {
    unsafe {
        tpde_initialize_session();
    }
}

pub fn emit_phi(dest: &str, incoming: Vec<(&str, i32)>) {
    let c_dest = CString::new(dest).expect("Failed to create CString");
    
    let c_blocks: Vec<CString> = incoming
        .iter()
        .map(|(b, _)| CString::new(*b).expect("Failed to create CString"))
        .collect();
    
    let block_ptrs: Vec<*const c_char> = c_blocks.iter().map(|b| b.as_ptr()).collect();
    let values: Vec<c_int> = incoming.iter().map(|(_, v)| *v as c_int).collect();

    unsafe {
        tpde_emit_phi_node(
            c_dest.as_ptr(),
            block_ptrs.as_ptr(),
            values.as_ptr(),
            incoming.len() as c_int,
        );
    }
}