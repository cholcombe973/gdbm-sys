pub const GDBM_READER: ::std::os::raw::c_uint = 0;
pub const GDBM_WRITER: ::std::os::raw::c_uint = 1;
pub const GDBM_WRCREAT: ::std::os::raw::c_uint = 2;
pub const GDBM_NEWDB: ::std::os::raw::c_uint = 3;
pub const GDBM_FAST: ::std::os::raw::c_uint = 16;
pub const GDBM_SYNC: ::std::os::raw::c_uint = 32;
pub const GDBM_NOLOCK: ::std::os::raw::c_uint = 64;

pub const GDBM_INSERT: ::std::os::raw::c_uint = 0;
pub const GDBM_REPLACE: ::std::os::raw::c_uint = 1;
pub const GDBM_CACHESIZE: ::std::os::raw::c_uint = 1;
pub const GDBM_FASTMODE: ::std::os::raw::c_uint = 2;
pub const GDBM_SYNCMODE: ::std::os::raw::c_uint = 3;
pub const GDBM_CENTFREE: ::std::os::raw::c_uint = 4;
pub const GDBM_COALESCEBLKS: ::std::os::raw::c_uint = 5;

pub const GDBM_NO_ERROR: ::std::os::raw::c_uint = 0;
pub const GDBM_MALLOC_ERROR: ::std::os::raw::c_uint = 1;
pub const GDBM_BLOCK_SIZE_ERROR: ::std::os::raw::c_uint = 2;
pub const GDBM_FILE_OPEN_ERROR: ::std::os::raw::c_uint = 3;
pub const GDBM_FILE_WRITE_ERROR: ::std::os::raw::c_uint = 4;
pub const GDBM_FILE_SEEK_ERROR: ::std::os::raw::c_uint = 5;
pub const GDBM_FILE_READ_ERROR: ::std::os::raw::c_uint = 6;
pub const GDBM_BAD_MAGIC_NUMBER: ::std::os::raw::c_uint = 7;
pub const GDBM_EMPTY_DATABASE: ::std::os::raw::c_uint = 8;
pub const GDBM_CANT_BE_READER: ::std::os::raw::c_uint = 9;
pub const GDBM_CANT_BE_WRITER: ::std::os::raw::c_uint = 10;
pub const GDBM_READER_CANT_DELETE: ::std::os::raw::c_uint = 11;
pub const GDBM_READER_CANT_STORE: ::std::os::raw::c_uint = 12;
pub const GDBM_READER_CANT_REORGANIZE: ::std::os::raw::c_uint = 13;
pub const GDBM_UNKNOWN_UPDATE: ::std::os::raw::c_uint = 14;
pub const GDBM_ITEM_NOT_FOUND: ::std::os::raw::c_uint = 15;
pub const GDBM_REORGANIZE_FAILED: ::std::os::raw::c_uint = 16;
pub const GDBM_CANNOT_REPLACE: ::std::os::raw::c_uint = 17;
pub const GDBM_ILLEGAL_DATA: ::std::os::raw::c_uint = 18;
pub const GDBM_OPT_ALREADY_SET: ::std::os::raw::c_uint = 19;
pub const GDBM_OPT_ILLEGAL: ::std::os::raw::c_uint = 20;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct datum {
    pub dptr: *mut ::std::os::raw::c_char,
    pub dsize: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_datum() {
    assert_eq!(::std::mem::size_of::<datum>(),
               16usize,
               concat!("Size of: ", stringify!(datum)));
    assert_eq!(::std::mem::align_of::<datum>(),
               8usize,
               concat!("Alignment of ", stringify!(datum)));
}
impl Clone for datum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _bindgen_ty_1 {
    pub dummy: [::std::os::raw::c_int; 10usize],
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_bindgen_ty_1>(),
               40usize,
               concat!("Size of: ", stringify!(_bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<_bindgen_ty_1>(),
               4usize,
               concat!("Alignment of ", stringify!(_bindgen_ty_1)));
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GDBM_FILE = *mut _bindgen_ty_1;
pub type gdbm_error = ::std::os::raw::c_int;

#[cfg(target_os = "linux")]
#[link(name = "gdbm", kind="dylib")]
extern "C" {
    pub static mut gdbm_version: *mut ::std::os::raw::c_char;
    pub fn gdbm_open(arg1: *mut ::std::os::raw::c_char,
                     arg2: ::std::os::raw::c_int,
                     arg3: ::std::os::raw::c_int,
                     arg4: ::std::os::raw::c_int,
                     arg5: ::std::option::Option<unsafe extern "C" fn()>)
                     -> GDBM_FILE;
    pub fn gdbm_close(arg1: GDBM_FILE);
    pub fn gdbm_store(arg1: GDBM_FILE,
                      arg2: datum,
                      arg3: datum,
                      arg4: ::std::os::raw::c_int)
                      -> ::std::os::raw::c_int;
    pub fn gdbm_fetch(arg1: GDBM_FILE, arg2: datum) -> datum;
    pub fn gdbm_delete(arg1: GDBM_FILE, arg2: datum) -> ::std::os::raw::c_int;
    pub fn gdbm_firstkey(arg1: GDBM_FILE) -> datum;
    pub fn gdbm_nextkey(arg1: GDBM_FILE, arg2: datum) -> datum;
    pub fn gdbm_reorganize(arg1: GDBM_FILE) -> ::std::os::raw::c_int;
    pub fn gdbm_sync(arg1: GDBM_FILE);
    pub fn gdbm_exists(arg1: GDBM_FILE, arg2: datum) -> ::std::os::raw::c_int;
    pub fn gdbm_setopt(arg1: GDBM_FILE,
                       arg2: ::std::os::raw::c_int,
                       arg3: *mut ::std::os::raw::c_int,
                       arg4: ::std::os::raw::c_int)
                       -> ::std::os::raw::c_int;
    pub fn gdbm_fdesc(arg1: GDBM_FILE) -> ::std::os::raw::c_int;
    pub static mut gdbm_errno: gdbm_error;
    pub fn gdbm_strerror(arg1: gdbm_error) -> *mut ::std::os::raw::c_char;
}
