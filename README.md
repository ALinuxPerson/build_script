<div align="center">
    <h1><b>Build Script</b></h1>
    <a href="https://www.crates.io/crates/build_script">
        <img src="https://img.shields.io/crates/v/serde.svg">
    </a>
    <a href="https://www.docs.rs/build_script">
        <img src="https://docs.rs/build_script/badge.svg">
    </a>
    <p>A wrapper for build.rs instructions</p>
</div>

# Why?
I made this because I felt like the way you pass instructions to `build.rs` makes it very easy to make mistakes 
(especially when using strings) and it just felt odd that rust doesn't provide an api or an official external crate 
(like [`rand`](https://crates.io/crates/rand)).

# Installation
Add this to your `Cargo.toml`:
```toml
[build-dependencies]
build_script = "0.1.3"
```

# Examples
```rust
use build_script::{cargo_rustc_link_lib, cargo_rustc_link_search, BuildScript, Instruction, Value};

fn main() {
    // basic instructions    
    build_script::cargo_rerun_if_changed("something.txt");
    build_script::cargo_rerun_if_env_changed("PKG_CONFIG");
    build_script::cargo_rustc_link_lib("somelibrary");
    build_script::cargo_rustc_link_lib_mapping(cargo_rustc_link_lib::Kind::DynamicLibrary, "somelibrary");
    build_script::cargo_rustc_link_search("something-else.txt");
    build_script::cargo_rustc_link_search_mapping(cargo_rustc_link_search::Kind::Crate, "something-else.txt");
    build_script::cargo_rustc_flags("-l ffi");
    build_script::cargo_rustc_cfg("key");
    build_script::cargo_rustc_cfg_mapping("key", "value");
    build_script::cargo_rustc_env("var", "value");
    build_script::cargo_rustc_cdylib_link_arg("flag");
    build_script::cargo_mapping("key", "value");

    // other, advanced instructions    
    let mut build_script = BuildScript::default();
    let instruction = {
        let value = Value::Singular("something".into());
        Instruction::new("instruction", value)
    };

    // add a custom instruction to the instruction stack    
    build_script.custom_instruction(instruction);

    // write all instructions to something (for this scenario, and also usually, its stdout)    
    build_script.build();
}
```

For more information see the documentation.

# Terminology
## Instruction
The instruction is what is passed to cargo. An example would be `cargo:rerun-if-env-changed=ENV`. This example will
also be dissected below.

The instruction is split into three parts:
### Prefix
The prefix is the string before the delimiter `:`: `cargo`.

Usually the prefix is `cargo`, however in this crate other, custom prefixes can be used for future compatibility in case
another prefix is added.

### Name
The name is the string in between the delimiters `:` and `=`: `rerun-if-env-changed`.

If the name is unknown, the instruction will automatically be a mapping (see below).
### Value
The value is the string after the delimiter `=`: `ENV`.

This represents the value of the instruction.
## Mapping
There is a type of instruction which is a mapping: `cargo:KEY=VALUE`.

This is, verbatim:

> Metadata, used by `links` scripts.

The source can be found [here](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

This is used when an instruction name is unknown.
