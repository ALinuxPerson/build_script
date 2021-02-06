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

fn lock_mutex<T>(lock: LockResult<MutexGuard<T>>) -> MutexGuard<T> {
    lock.expect("mutex is poisoned")
}

fn build_script() -> MutexGuard<'static, BuildScript<Stdout>> {
    lock_mutex(BUILD_SCRIPT.lock())
}

pub fn cargo_rerun_if_changed(path: impl Into<PathBuf>) {
    build_script().cargo_rerun_if_changed(path.into());
}

pub fn cargo_rerun_if_env_changed(var: impl Into<String>) {
    build_script().cargo_rerun_if_env_changed(&var.into());
}

pub fn cargo_rustc_link_lib(name: impl Into<String>) {
    build_script().cargo_rustc_link_lib(None, &name.into());
}

pub fn cargo_rustc_link_lib_mapping(kind: cargo_rustc_link_lib_::Kind, name: impl Into<String>) {
    build_script().cargo_rustc_link_lib(kind.into(), &name.into());
}

pub fn cargo_rustc_link_search(path: impl Into<PathBuf>) {
    build_script().cargo_rustc_link_search(None, path.into());
}

pub fn cargo_rustc_link_search_mapping(
    kind: cargo_rustc_link_search_::Kind,
    path: impl Into<PathBuf>,
) {
    build_script().cargo_rustc_link_search(kind.into(), path.into());
}

pub fn cargo_rustc_flags(flags: impl Into<String>) {
    build_script().cargo_rustc_flags(&flags.into());
}

pub fn cargo_rustc_cfg(key: impl Into<String>) {
    build_script().cargo_rustc_cfg(&key.into(), None);
}

pub fn cargo_rustc_cfg_mapping(key: impl Into<String>, value: impl Into<String>) {
    build_script().cargo_rustc_cfg(&key.into(), Some(&value.into()));
}

pub fn cargo_rustc_env(var: impl Into<String>, value: impl Into<String>) {
    build_script().cargo_rustc_env(&var.into(), &value.into());
}

pub fn cargo_rustc_cdylib_link_arg(flag: impl Into<String>) {
    build_script().cargo_rustc_cdylib_link_arg(&flag.into());
}

pub fn cargo_warning(message: impl Into<String>) {
    build_script().cargo_warning(&message.into());
}

pub fn cargo_mapping(key: impl Into<String>, value: impl Into<String>) {
    build_script().cargo_mapping(&key.into(), &value.into());
}
