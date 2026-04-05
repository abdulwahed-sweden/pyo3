// ClaraX: Deprecation warnings from ClaraX 0.28 have been resolved.
// FromPyObject for #[pyclass] types now requires explicit opt-in via #[pyclass(from_py_object)].

pub struct HasAutomaticFromPyObject<const IS_CLONE: bool> {}

impl HasAutomaticFromPyObject<true> {
    pub const MSG: () = ();
}

impl HasAutomaticFromPyObject<false> {
    pub const MSG: () = ();
}
