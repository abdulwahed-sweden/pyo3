// Brings in `test_utils` from the `tests` directory
//
// to make that file function (lots of references to `clarax` within it) need
// re-bind `crate` as clarax
use crate as clarax;
include!("../tests/test_utils/mod.rs");
