use std::env;

use pyforge_build_config::pyo3_build_script_impl::{cargo_env_var, errors::Result};
use pyforge_build_config::{
    add_python_framework_link_args, bail, print_feature_cfgs, InterpreterConfig,
};

fn ensure_auto_initialize_ok(interpreter_config: &InterpreterConfig) -> Result<()> {
    if cargo_env_var("CARGO_FEATURE_AUTO_INITIALIZE").is_some() && !interpreter_config.shared {
        bail!(
            "The `auto-initialize` feature is enabled, but your python installation only supports \
            embedding the Python interpreter statically. If you are attempting to run tests, or a \
            binary which is okay to link dynamically, install a Python distribution which ships \
            with the Python shared library.\n\
            \n\
            Embedding the Python interpreter statically does not yet have first-class support in \
            PyForge. If you are sure you intend to do this, disable the `auto-initialize` feature.\n\
            \n\
            For more information, see \
            https://github.com/abdulwahed-sweden/pyforge/v{pyforge_version}/\
                building-and-distribution.html#embedding-python-in-rust",
            pyforge_version = env::var("CARGO_PKG_VERSION").unwrap()
        );
    }
    Ok(())
}

/// Prepares the PyForge crate for compilation.
///
/// This loads the config from pyo3-build-config and then makes some additional checks to improve UX
/// for users.
///
/// Emits the cargo configuration based on this config as well as a few checks of the Rust compiler
/// version to enable features which aren't supported on MSRV.
fn configure_pyforge() -> Result<()> {
    let interpreter_config = pyforge_build_config::get();

    ensure_auto_initialize_ok(interpreter_config)?;

    for cfg in interpreter_config.build_script_outputs() {
        println!("{cfg}")
    }

    print_feature_cfgs();

    // Make `cargo test` etc work on macOS with Xcode bundled Python
    add_python_framework_link_args();

    Ok(())
}

fn main() {
    pyforge_build_config::print_expected_cfgs();
    if let Err(e) = configure_pyforge() {
        eprintln!("error: {}", e.report());
        std::process::exit(1)
    }
}
