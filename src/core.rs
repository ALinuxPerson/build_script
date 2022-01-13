//! Core functionality for [`build_script`](crate).
//! # Notes
//! 99% of the time, you won't need to use this module. Instead, use the [`basic`](crate::basic)
//! module instead.
use crate::cargo_rustc_link_lib;
use crate::cargo_rustc_link_search;
use crate::utils::VecExt;
use crate::{Instruction, Value};
use std::io::Stdout;
use std::path::PathBuf;
use std::{io, str};

/// A build script. This is the main struct for creating cargo arguments.
/// # Notes
/// 99% of the time, you won't need this. Instead, use the functions in [`basic`](crate::basic).
pub struct BuildScript<'w> {
    /// The instruction stack. If `now` is `true`, this will not be used.
    instructions: Vec<Instruction>,

    /// Whether or not you should write instructions to `writer` immediately.
    now: bool,

    /// The writer where instructions will be written.
    /// # Notes
    /// 99% of the time, you can use the defaults, which is [`io::stdout()`](io::stdout).
    writer: &'w mut (dyn io::Write + Send),
}

impl Default for BuildScript<'static> {
    /// Get the default build script. Writer is [`io::stdout()`](io::stdout).
    /// # Notes
    /// 99% of the time, you can use this associated function instead.
    fn default() -> Self {
        let stdout = Box::new(io::stdout());
        let stdout = Box::leak(stdout);
        Self::new(stdout)
    }
}

impl<'w> BuildScript<'w> {
    /// Create a new [`BuildScript`](Self).
    /// # Notes
    /// 99% of the time, you won't need to yse this associated function. The defaults can be used instead
    /// ([`BuildScript::default()`](Self::default)).
    pub fn new(writer: &'w mut (dyn io::Write + Send)) -> Self {
        Self {
            writer,
            instructions: Vec::new(),
            now: false,
        }
    }
    /// Sets `now` to true.
    pub fn now(&mut self) -> &mut Self {
        self.now = true;

        self
    }

    /// Write to `writer`.
    fn write(&mut self, string: &str) {
        /// Newline.
        const NEWLINE: u8 = b'\n';

        let string = {
            let mut bytes = string.as_bytes().to_vec();

            if let Some(last) = bytes.last() {
                if last != &NEWLINE {
                    bytes.push(NEWLINE)
                }
            }

            String::from_utf8(bytes)
                .expect("string contained invalid utf8 even if it was already a string before")
        };

        write!(self.writer, "{}", string).expect("failed to write to writer")
    }

    /// Write the instruction immediately if `now` is true, else push it to the instruction stack.
    fn parse_instruction(&mut self, instruction: Instruction) {
        if self.now {
            self.write(&instruction.to_string())
        } else {
            self.instructions.push(instruction)
        }
    }

    /// Write and remove all the instructions in the stack, starting from the first.
    pub fn build(&mut self) {
        while let Some(instruction) = self.instructions.take_first() {
            self.write(&instruction.to_string())
        }
    }

    /// Wrapper for `cargo:rerun-if-changed=PATH`. This tells Cargo when to rerun the script.
    pub fn cargo_rerun_if_changed(&mut self, path: PathBuf) -> &mut Self {
        let instruction = Instruction::new(
            "rerun-if-changed",
            Value::Singular(path.display().to_string()),
        );

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rerun-if-env-changed=VAR`. This tells Cargo when to rerun the script.
    pub fn cargo_rerun_if_env_changed(&mut self, var: &str) -> &mut Self {
        let instruction = Instruction::new("rerun-if-env-changed", Value::Singular(var.into()));

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rustc-link-lib=[KIND=]NAME`. This adds a library to link.
    pub fn cargo_rustc_link_lib(
        &mut self,
        kind: Option<cargo_rustc_link_lib::Kind>,
        name: &str,
    ) -> &mut Self {
        let instruction = Instruction::new(
            "rustc-link-lib",
            Value::UnquotedOptionalKey(kind.map(Into::into), name.into()),
        );

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rustc-link-search=[KIND=]PATH`. This adds to the library search path.
    pub fn cargo_rustc_link_search(
        &mut self,
        kind: Option<cargo_rustc_link_search::Kind>,
        path: PathBuf,
    ) -> &mut Self {
        let instruction = Instruction::new(
            "rustc-link-search",
            Value::UnquotedOptionalKey(kind.map(Into::into), path.display().to_string()),
        );

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rustc-flags=FLAGS`. This passes certain flags to the compiler.
    pub fn cargo_rustc_flags(&mut self, flags: &str) -> &mut Self {
        let instruction = Instruction::new("rustc-flags", Value::Singular(flags.into()));

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rustc-cfg=KEY[="VALUE"]`. This enable compile-time `cfg` settings.
    pub fn cargo_rustc_cfg(&mut self, key: &str, value: Option<&str>) -> &mut Self {
        let instruction = Instruction::new(
            "rustc-cfg",
            Value::OptionalValue(key.into(), value.map(Into::into)),
        );

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rustc-env=VAR=VALUE`. This sets an environment variable.
    pub fn cargo_rustc_env(&mut self, var: &str, value: &str) -> &mut Self {
        let instruction = Instruction::new(
            "rustc-env",
            Value::UnquotedMapping(var.into(), value.into()),
        );

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:rustc-cdylib-link-arg=FLAG`. This passes custom flags to a linker for
    /// cdylib crates.
    pub fn cargo_rustc_cdylib_link_arg(&mut self, flag: &str) -> &mut Self {
        let instruction = Instruction::new("rustc-cdylib-link-arg", Value::Singular(flag.into()));

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:warning=MESSAGE`. This displays a warning on the terminal.
    pub fn cargo_warning(&mut self, message: &str) -> &mut Self {
        let instruction = Instruction::new("warning", Value::Singular(message.into()));

        self.custom_instruction(instruction)
    }

    /// Wrapper for `cargo:KEY=VALUE`. This is metadata, used by `links` scripts.
    pub fn cargo_mapping(&mut self, key: &str, value: &str) -> &mut Self {
        let instruction =
            Instruction::new_mapping(Value::UnquotedMapping(key.into(), value.into()));

        self.custom_instruction(instruction)
    }

    /// Pass a custom instruction. Internally, [`BuildScript`](Self) uses this. This may be used
    /// when `build_script` isn't updated for new instructions yet in the future.
    pub fn custom_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.parse_instruction(instruction);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::BuildScript;
    use crate::{Instruction, Value};

    fn parse_bytes_to_lines(bytes: &[u8]) -> Vec<String> {
        let bytes = String::from_utf8_lossy(bytes).to_string();
        bytes.lines().map(str::to_owned).collect()
    }

    #[test]
    fn test_new() {
        let mut writer = Vec::new();
        {
            let mut build_script = BuildScript::new(&mut writer);
            build_script.build();
            assert!(writer.is_empty());
        }
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_mapping("key", "value");
        assert!(writer.is_empty());
    }

    #[test]
    fn test_now() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.now();
        build_script.cargo_mapping("key", "value");
        let output = &parse_bytes_to_lines(&writer)[0];
        assert_eq!(output, "cargo:key=value")
    }

    #[test]
    fn test_build() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_mapping("key", "value");
        build_script.cargo_mapping("second", "mapping");
        build_script.build();
        let lines = parse_bytes_to_lines(&writer);
        let expected = vec![
            "cargo:key=value".to_string(),
            "cargo:second=mapping".to_string(),
        ];
        assert_eq!(lines, expected)
    }

    #[test]
    fn test_cargo_rerun_if_changed() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script
            .cargo_rerun_if_changed("library.h".into())
            .build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:rerun-if-changed=library.h";
        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rerun_if_env_changed() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rerun_if_env_changed("ENV").build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:rerun-if-env-changed=ENV";
        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rustc_link_lib() {
        use crate::cargo_rustc_link_lib::Kind;

        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rustc_link_lib(None, "first");
        build_script.cargo_rustc_link_lib(Kind::Framework.into(), "second");
        build_script.cargo_rustc_link_lib(Kind::Static.into(), "third");
        build_script.cargo_rustc_link_lib(Kind::DynamicLibrary.into(), "fourth");
        build_script.build();
        let output = parse_bytes_to_lines(&writer);
        let expected = vec![
            "cargo:rustc-link-lib=first",
            "cargo:rustc-link-lib=framework=second",
            "cargo:rustc-link-lib=static=third",
            "cargo:rustc-link-lib=dylib=fourth",
        ];

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rustc_link_search() {
        use crate::cargo_rustc_link_search::Kind;

        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rustc_link_search(Kind::Framework.into(), "first".into());
        build_script.cargo_rustc_link_search(Kind::All.into(), "second".into());
        build_script.cargo_rustc_link_search(Kind::Native.into(), "third".into());
        build_script.cargo_rustc_link_search(Kind::Crate.into(), "fourth".into());
        build_script.cargo_rustc_link_search(Kind::Dependency.into(), "fifth".into());
        build_script.cargo_rustc_link_search(None, "sixth".into());
        build_script.build();
        let output = parse_bytes_to_lines(&writer);
        let expected = vec![
            "cargo:rustc-link-search=framework=first",
            "cargo:rustc-link-search=all=second",
            "cargo:rustc-link-search=native=third",
            "cargo:rustc-link-search=crate=fourth",
            "cargo:rustc-link-search=dependency=fifth",
            "cargo:rustc-link-search=sixth",
        ];

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rustc_flags() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rustc_flags("flags").build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:rustc-flags=flags";

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rustc_cfg() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rustc_cfg("key", None);
        build_script.cargo_rustc_cfg("key", "value".into());
        build_script.build();
        let output = parse_bytes_to_lines(&writer);
        let expected = vec!["cargo:rustc-cfg=key", "cargo:rustc-cfg=key=\"value\""];

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rustc_env() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rustc_env("var", "value").build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:rustc-env=var=value";

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_rustc_cdylib_link_arg() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_rustc_cdylib_link_arg("flag").build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:rustc-cdylib-link-arg=flag";

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_warning() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_warning("message").build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:warning=message";

        assert_eq!(output, expected)
    }

    #[test]
    fn test_cargo_mapping() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        build_script.cargo_mapping("key", "value").build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:key=value";

        assert_eq!(output, expected)
    }

    #[test]
    fn test_custom_instruction() {
        let mut writer = Vec::new();
        let mut build_script = BuildScript::new(&mut writer);
        let instruction =
            Instruction::new("some-instruction", Value::Singular("Hello, World!".into()));
        build_script.custom_instruction(instruction).build();
        let output = &parse_bytes_to_lines(&writer)[0];
        let expected = "cargo:some-instruction=Hello, World!";

        assert_eq!(output, expected)
    }
}
