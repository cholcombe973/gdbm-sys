extern crate gdbm_sys;

#[test]
fn test_open() {
    use std::ffi::CString;
    let path = CString::new("test".as_bytes()).unwrap();
    unsafe {
        let db_ptr = gdbm_sys::gdbm_open(path.as_ptr() as *mut i8, 0, 0, 0, None);
    }
}
