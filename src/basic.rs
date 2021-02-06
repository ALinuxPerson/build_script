//! The most basic usage for [`build_script`](crate).
//! # Notes
//! 99% of the time, all of the public functions in this crate can suffice.
use crate::BuildScript;
use crate::{
    cargo_rustc_link_lib as cargo_rustc_link_lib_,
    cargo_rustc_link_search as cargo_rustc_link_search_,
};
use lazy_static::lazy_static;
use std::io::Stdout;
use std::path::PathBuf;
use std::sync::{LockResult, Mutex, MutexGuard};

lazy_static! {
    static ref BUILD_SCRIPT: Mutex<BuildScript<Stdout>> = {
        let mut build_script = BuildScript::default();
        build_script.now();

        Mutex::new(build_script)
    };
}

/// Lock the mutex of [`BUILD_SCRIPT`](static@BUILD_SCRIPT). This panics if the mutex is poisoned.
fn lock_mutex<T>(lock: LockResult<MutexGuard<T>>) -> MutexGuard<T> {
    lock.expect("mutex is poisoned")
}

/// Wrapper for locking the mutex in [`BUILD_SCRIPT`](static@BUILD_SCRIPT). Internally this handles locking
/// the mutex of [`BUILD_SCRIPT`](statc@BUILD_SCRIPT) and then panicking if mutex is poisoned.
fn build_script() -> MutexGuard<'static, BuildScript<Stdout>> {
    lock_mutex(BUILD_SCRIPT.lock())
}

/// Wrapper for `cargo:rerun-if-changed=PATH`. This tells Cargo when to rerun the script.
pub fn cargo_rerun_if_changed(path: impl Into<PathBuf>) {
    build_script().cargo_rerun_if_changed(path.into());
}

/// Wrapper for `cargo:rerun-if-env-changed=VAR`. This tells Cargo when to rerun the script.
pub fn cargo_rerun_if_env_changed(var: impl Into<String>) {
    build_script().cargo_rerun_if_env_changed(&var.into());
}

/// Wrapper for `cargo:rustc-link-lib=[KIND=]NAME`. This adds a library to link.
pub fn cargo_rustc_link_lib(name: impl Into<String>) {
    build_script().cargo_rustc_link_lib(None, &name.into());
}

/// [`cargo_rustc_link_lib()`](cargo_rustc_link_lib), but with the `kind` parameter needed.
pub fn cargo_rustc_link_lib_mapping(kind: cargo_rustc_link_lib_::Kind, name: impl Into<String>) {
    build_script().cargo_rustc_link_lib(kind.into(), &name.into());
}

/// Wrapper for `cargo:rustc-link-search=[KIND=]PATH`. This adds to the library search path.
pub fn cargo_rustc_link_search(path: impl Into<PathBuf>) {
    build_script().cargo_rustc_link_search(None, path.into());
}

/// [`cargo_rustc_link_search()`](cargo_rustc_link_search), but with the `kind` parameter needed.
pub fn cargo_rustc_link_search_mapping(
    kind: cargo_rustc_link_search_::Kind,
    path: impl Into<PathBuf>,
) {
    build_script().cargo_rustc_link_search(kind.into(), path.into());
}

/// Wrapper for `cargo:rustc-flags=FLAGS`. This passes certain flags to the compiler.
pub fn cargo_rustc_flags(flags: impl Into<String>) {
    build_script().cargo_rustc_flags(&flags.into());
}

/// Wrapper for `cargo:rustc-cfg=KEY[="VALUE"]`. This enable compile-time `cfg` settings.
pub fn cargo_rustc_cfg(key: impl Into<String>) {
    build_script().cargo_rustc_cfg(&key.into(), None);
}

/// [`cargo_rustc_cfg()`](cargo_rustc_cfg), but with the `value` parameter needed.
pub fn cargo_rustc_cfg_mapping(key: impl Into<String>, value: impl Into<String>) {
    build_script().cargo_rustc_cfg(&key.into(), Some(&value.into()));
}

/// Wrapper for `cargo:rustc-env=VAR=VALUE`. This sets an environment variable.
pub fn cargo_rustc_env(var: impl Into<String>, value: impl Into<String>) {
    build_script().cargo_rustc_env(&var.into(), &value.into());
}

/// Wrapper for `cargo:rustc-cdylib-link-arg=FLAG`. This passes custom flags to a linker for cdylib
/// crates.
pub fn cargo_rustc_cdylib_link_arg(flag: impl Into<String>) {
    build_script().cargo_rustc_cdylib_link_arg(&flag.into());
}

/// Wrapper for `cargo:warning=MESSAGE`. This displays a warning on the terminal.
pub fn cargo_warning(message: impl Into<String>) {
    build_script().cargo_warning(&message.into());
}

/// Wrapper for `cargo:KEY=VALUE`. This is metadata, used by `links` scripts.
pub fn cargo_mapping(key: impl Into<String>, value: impl Into<String>) {
    build_script().cargo_mapping(&key.into(), &value.into());
}
