use std::ffi::c_int;

extern_libpython! {
    pub fn PyOS_InterruptOccurred() -> c_int;
    pub fn PyOS_BeforeFork();
    pub fn PyOS_AfterFork_Parent();
    pub fn PyOS_AfterFork_Child();
    #[deprecated(note = "use PyOS_AfterFork_Child instead")]
    pub fn PyOS_AfterFork();

    // skipped non-limited _PyOS_IsMainThread
    // skipped non-limited Windows _PyOS_SigintEvent
}
