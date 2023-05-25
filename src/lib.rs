//! A wrapper for `build.rs` instructions.
//! # Examples
//! ```rust
//! use build_script::{cargo_rustc_link_lib, cargo_rustc_link_search, BuildScript, Instruction, Value};
//!
//! // basic instructions
//! build_script::cargo_rerun_if_changed("something.txt");
//! build_script::cargo_rerun_if_env_changed("PKG_CONFIG");
//! build_script::cargo_rustc_link_lib("somelibrary");
//! build_script::cargo_rustc_link_lib_mapping(cargo_rustc_link_lib::Kind::DynamicLibrary, "somelibrary");
//! build_script::cargo_rustc_link_search("something-else.txt");
//! build_script::cargo_rustc_link_search_mapping(cargo_rustc_link_search::Kind::Crate, "something-else.txt");
//! build_script::cargo_rustc_flags("-l ffi");
//! build_script::cargo_rustc_cfg("key");
//! build_script::cargo_rustc_cfg_mapping("key", "value");
//! build_script::cargo_rustc_env("var", "value");
//! build_script::cargo_rustc_cdylib_link_arg("flag");
//! build_script::cargo_mapping("key", "value");
//!
//! // other, advanced instructions
//! let mut build_script = BuildScript::default();
//! let instruction = {
//!     let value = Value::Singular("something".into());
//!     Instruction::new("instruction", value)
//! };
//!
//! // add a custom instruction to the instruction stack
//! build_script.custom_instruction(instruction);
//!
//! // write all instructions to something (for this scenario, and also usually, its stdout)
//! build_script.build();
//! ```
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
pub mod basic;
pub mod cargo_rustc_link_lib;
pub mod cargo_rustc_link_search;
pub mod core;
pub mod env;
pub mod instruction;
pub mod prefix;
mod utils;
pub mod value;

pub use self::core::BuildScript;
pub use basic::*;
pub use instruction::Instruction;
pub use prefix::Prefix;
pub use value::Value;
