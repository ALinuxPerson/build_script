//! A wrapper for environment variables set by `cargo` for build scripts
//!
//! See [the Rust docs](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
//! for more information.

use std::env::{var, VarError};

/// Path to the cargo binary performing the build.
const CARGO: &str = "CARGO";
/// The directory containing the manifest for the package being built (the package containing the build script). Also note that this is the value of the current working directory of the build script when it starts.
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
/// the manifest links value.
const CARGO_MANIFEST_LINKS: &str = "CARGO_MANIFEST_LINKS";
/// Contains parameters needed for Cargo’s jobserver implementation to parallelize subprocesses. Rustc or cargo invocations from build.rs can already read CARGO_MAKEFLAGS, but GNU Make requires the flags to be specified either directly as arguments, or through the MAKEFLAGS environment variable. Currently Cargo doesn’t set the MAKEFLAGS variable, but it’s free for build scripts invoking GNU Make to set it to the contents of CARGO_MAKEFLAGS.
const CARGO_MAKEFLAGS: &str = "CARGO_MAKEFLAGS";
/// Set on UNIX-like platforms.
const CARGO_CFG_UNIX: &str = "CARGO_CFG_UNIX";
/// Set on windows-like platforms.
const CARGO_CFG_WINDOWS: &str = "CARGO_CFG_WINDOWS";
/// The target family.
const CARGO_CFG_TARGET_FAMILY: &str = "CARGO_CFG_TARGET_FAMILY";
/// The target operating system.
const CARGO_CFG_TARGET_OS: &str = "CARGO_CFG_TARGET_OS";
/// The CPU target architecture.
const CARGO_CFG_TARGET_ARCH86_64: &str = "CARGO_CFG_TARGET_ARCH86_64";
/// The target vendor.
const CARGO_CFG_TARGET_VENDOR: &str = "CARGO_CFG_TARGET_VENDOR";
/// The target environment ABI.
const CARGO_CFG_TARGET_ENV: &str = "CARGO_CFG_TARGET_ENV";
/// The CPU pointer width.
const CARGO_CFG_TARGET_POINTER_WIDTH: &str = "CARGO_CFG_TARGET_POINTER_WIDTH";
/// The CPU target endianness.
const CARGO_CFG_TARGET_ENDIAN: &str = "CARGO_CFG_TARGET_ENDIAN";
/// List of CPU target features enabled.
const CARGO_CFG_TARGET_FEATURE: &str = "CARGO_CFG_TARGET_FEATURE";
/// the folder in which all output and intermediate artifacts should be placed. This folder is inside the build directory for the package being built, and it is unique for the package in question.
const OUT_DIR: &str = "OUT_DIR";
/// the target triple that is being compiled for. Native code should be compiled for this triple. See the Target Triple description for more information.
const TARGET: &str = "TARGET";
/// the host triple of the Rust compiler.
const HOST: &str = "HOST";
/// the parallelism specified as the top-level parallelism. This can be useful to pass a -j parameter to a system like make. Note that care should be taken when interpreting this environment variable. For historical purposes this is still provided but recent versions of Cargo, for example, do not need to run make -j, and instead can set the MAKEFLAGS env var to the content of CARGO_MAKEFLAGS to activate the use of Cargo’s GNU Make compatible jobserver for sub-make invocations.
const NUM_JOBS: &str = "NUM_JOBS";
/// — values of the corresponding variables for the profile currently being built.
const OPT_LEVEL: &str = "OPT_LEVEL";
/// Set when building with debug profile
const DEBUG: &str = "DEBUG";
/// release for release builds, debug for other builds. This is determined based on if the profile inherits from the dev or release profile. Using this environment variable is not recommended. Using other environment variables like OPT_LEVEL provide a more correct view of the actual settings being used.
const PROFILE: &str = "PROFILE";
/// The compiler that Cargo has resolved to use, passed to the build script so it might use it as well.
const RUSTC: &str = "RUSTC";
/// The documentation generator that Cargo has resolved to use, passed to the build script so it might use it as well.
const RUSTDOC: &str = "RUSTDOC";
/// the rustc wrapper, if any, that Cargo is using. See build.rustc-wrapper.
const RUSTC_WRAPPER: &str = "RUSTC_WRAPPER";
/// the rustc wrapper, if any, that Cargo is using for workspace members. See build.rustc-workspace-wrapper.
const RUSTC_WORKSPACE_WRAPPER: &str = "RUSTC_WORKSPACE_WRAPPER";
/// The path to the linker binary that Cargo has resolved to use for the current target, if specified. The linker can be changed by editing .cargo/config.toml; see the documentation about cargo configuration for more information.
const RUSTC_LINKER: &str = "RUSTC_LINKER";
/// extra flags that Cargo invokes rustc with, separated by a 0x1f character (ASCII Unit Separator). See build.rustflags. Note that since Rust 1.55, RUSTFLAGS is removed from the environment; scripts should use CARGO_ENCODED_RUSTFLAGS instead.
const CARGO_ENCODED_RUSTFLAGS: &str = "CARGO_ENCODED_RUSTFLAGS";

/// The path to the cargo command
pub fn cargo_var_cargo() -> Result<String, VarError> {
    var(CARGO)
}
/// The directory containing the manifest for the package being built (the package containing the build script). Also note that this is the value of the current working directory of the build script when it starts.
pub fn cargo_var_cargo_manifest_dir() -> Result<String, VarError> {
    var(CARGO_MANIFEST_DIR)
}
/// the manifest links value.
pub fn cargo_var_cargo_manifest_links() -> Result<String, VarError> {
    var(CARGO_MANIFEST_LINKS)
}
/// Contains parameters needed for Cargo’s jobserver implementation to parallelize subprocesses. Rustc or cargo invocations from build.rs can already read CARGO_MAKEFLAGS, but GNU Make requires the flags to be specified either directly as arguments, or through the MAKEFLAGS environment variable. Currently Cargo doesn’t set the MAKEFLAGS variable, but it’s free for build scripts invoking GNU Make to set it to the contents of CARGO_MAKEFLAGS.
pub fn cargo_var_cargo_makeflags() -> Result<String, VarError> {
    var(CARGO_MAKEFLAGS)
}
/// Set on UNIX-like platforms.
pub fn cargo_var_cargo_cfg_unix() -> Result<String, VarError> {
    var(CARGO_CFG_UNIX)
}
/// Set on windows-like platforms.
pub fn cargo_var_cargo_cfg_windows() -> Result<String, VarError> {
    var(CARGO_CFG_WINDOWS)
}
/// The target family.
pub fn cargo_var_cargo_cfg_target_family() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_FAMILY)
}
/// The target operating system.
pub fn cargo_var_cargo_cfg_target_os() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_OS)
}
/// The CPU target architecture.
pub fn cargo_var_cargo_cfg_target_arch86_64() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_ARCH86_64)
}
/// The target vendor.
pub fn cargo_var_cargo_cfg_target_vendor() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_VENDOR)
}
/// The target environment ABI.
pub fn cargo_var_cargo_cfg_target_env() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_ENV)
}
/// The CPU pointer width.
pub fn cargo_var_cargo_cfg_target_pointer_width() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_POINTER_WIDTH)
}
/// The CPU target endianness.
pub fn cargo_var_cargo_cfg_target_endian() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_ENDIAN)
}
/// List of CPU target features enabled.
pub fn cargo_var_cargo_cfg_target_feature() -> Result<String, VarError> {
    var(CARGO_CFG_TARGET_FEATURE)
}
/// the folder in which all output and intermediate artifacts should be placed. This folder is inside the build directory for the package being built, and it is unique for the package in question.
pub fn cargo_var_out_dir() -> Result<String, VarError> {
    var(OUT_DIR)
}
/// the target triple that is being compiled for. Native code should be compiled for this triple. See the Target Triple description for more information.
pub fn cargo_var_target() -> Result<String, VarError> {
    var(TARGET)
}
/// the host triple of the Rust compiler.
pub fn cargo_var_host() -> Result<String, VarError> {
    var(HOST)
}
/// the parallelism specified as the top-level parallelism. This can be useful to pass a -j parameter to a system like make. Note that care should be taken when interpreting this environment variable. For historical purposes this is still provided but recent versions of Cargo, for example, do not need to run make -j, and instead can set the MAKEFLAGS env var to the content of CARGO_MAKEFLAGS to activate the use of Cargo’s GNU Make compatible jobserver for sub-make invocations.
pub fn cargo_var_num_jobs() -> Result<String, VarError> {
    var(NUM_JOBS)
}
/// — values of the corresponding variables for the profile currently being built.
pub fn cargo_var_opt_level() -> Result<String, VarError> {
    var(OPT_LEVEL)
}
/// Set when building with debug profile
pub fn cargo_var_debug() -> Result<String, VarError> {
    var(DEBUG)
}
/// release for release builds, debug for other builds. This is determined based on if the profile inherits from the dev or release profile. Using this environment variable is not recommended. Using other environment variables like OPT_LEVEL provide a more correct view of the actual settings being used.
pub fn cargo_var_profile() -> Result<String, VarError> {
    var(PROFILE)
}
/// The compiler that Cargo has resolved to use, passed to the build script so it might use it as well.
pub fn cargo_var_rustc() -> Result<String, VarError> {
    var(RUSTC)
}
/// The documentation generator that Cargo has resolved to use, passed to the build script so it might use it as well.
pub fn cargo_var_rustdoc() -> Result<String, VarError> {
    var(RUSTDOC)
}
/// the rustc wrapper, if any, that Cargo is using. See build.rustc-wrapper.
pub fn cargo_var_rustc_wrapper() -> Result<String, VarError> {
    var(RUSTC_WRAPPER)
}
/// the rustc wrapper, if any, that Cargo is using for workspace members. See build.rustc-workspace-wrapper.
pub fn cargo_var_rustc_workspace_wrapper() -> Result<String, VarError> {
    var(RUSTC_WORKSPACE_WRAPPER)
}
/// The path to the linker binary that Cargo has resolved to use for the current target, if specified. The linker can be changed by editing .cargo/config.toml; see the documentation about cargo configuration for more information.
pub fn cargo_var_rustc_linker() -> Result<String, VarError> {
    var(RUSTC_LINKER)
}
/// extra flags that Cargo invokes rustc with, separated by a 0x1f character (ASCII Unit Separator). See build.rustflags. Note that since Rust 1.55, RUSTFLAGS is removed from the environment; scripts should use CARGO_ENCODED_RUSTFLAGS instead.
pub fn cargo_var_cargo_encoded_rustflags() -> Result<String, VarError> {
    var(CARGO_ENCODED_RUSTFLAGS)
}

/// For each activated feature of the package being built, this environment variable will be present where <name> is the name of the feature uppercased and having - translated to _.
pub fn cargo_var_cargo_feature(name: &str) -> Result<String, VarError> {
    var(format!(
        "CARGO_FEATURE_{}",
        name.replace('-', "_").to_uppercase()
    ))
}

/// For each configuration option of the package being built, this environment variable will contain the value of the configuration, where <cfg> is the name of the configuration uppercased and having - translated to _. Boolean configurations are present if they are set, and not present otherwise. Configurations with multiple values are joined to a single variable with the values delimited by ,.
pub fn cargo_var_cargo_cfg(cfg: &str) -> Result<String, VarError> {
    var(format!(
        "CARGO_CFG_{}",
        cfg.replace('-', "_").to_uppercase()
    ))
}

/// The package information variables, with the same names and values as are provided during crate building.
pub fn cargo_var_cargo_pkg(variable: &str) -> Result<String, VarError> {
    var(format!(
        "CARGO_PKG_{}",
        variable.replace('-', "_").to_uppercase()
    ))
}
