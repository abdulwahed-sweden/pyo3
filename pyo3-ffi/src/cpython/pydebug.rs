use std::ffi::{c_char, c_int};

#[cfg(not(Py_LIMITED_API))]
extern_libpython! {
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_DebugFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_VerboseFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_QuietFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_InteractiveFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_InspectFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_OptimizeFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_NoSiteFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_BytesWarningFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_UseClassExceptionsFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_FrozenFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_IgnoreEnvironmentFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_DontWriteBytecodeFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_NoUserSiteDirectory: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_UnbufferedStdioFlag: c_int;
    pub static mut Py_HashRandomizationFlag: c_int;
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_IsolatedFlag: c_int;
    #[cfg(windows)]
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_LegacyWindowsFSEncodingFlag: c_int;
    #[cfg(windows)]
    #[deprecated(note = "Python 3.12")]
    pub static mut Py_LegacyWindowsStdioFlag: c_int;
}

extern_libpython! {
    pub fn Py_GETENV(name: *const c_char) -> *mut c_char;
}

