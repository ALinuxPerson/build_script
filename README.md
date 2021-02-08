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
