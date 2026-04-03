fn main() {
    pyforge_build_config::use_pyo3_cfgs();
    pyforge_build_config::add_libpython_rpath_link_args();
}
