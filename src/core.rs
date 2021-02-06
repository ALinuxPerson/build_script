use crate::cargo_rustc_link_lib;
use crate::cargo_rustc_link_search;
use crate::utils::VecExt;
use crate::{Instruction, Value};
use std::io::Stdout;
use std::path::PathBuf;
use std::{io, str};

pub struct BuildScript<W: io::Write + Send> {
    instructions: Vec<Instruction>,
    now: bool,
    writer: W,
}

impl Default for BuildScript<Stdout> {
    fn default() -> Self {
        Self::new(io::stdout())
    }
}

impl<W: io::Write + Send> BuildScript<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            instructions: Vec::new(),
            now: false,
        }
    }

    pub fn now(&mut self) -> &mut Self {
        self.now = true;

        self
    }

    fn write(&mut self, string: &str) {
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

    fn parse_instruction(&mut self, instruction: Instruction) {
        if self.now {
            self.write(&instruction.to_string())
        } else {
            self.instructions.push(instruction)
        }
    }

    pub fn build(&mut self) {
        while let Some(instruction) = self.instructions.take_first() {
            self.write(&instruction.to_string())
        }
    }

    pub fn cargo_rerun_if_changed(&mut self, path: PathBuf) -> &mut Self {
        let instruction = Instruction::new(
            "rerun-if-changed",
            Value::Singular(path.display().to_string()),
        );

        self.custom_instruction(instruction)
    }

    pub fn cargo_rerun_if_env_changed(&mut self, var: &str) -> &mut Self {
        let instruction = Instruction::new("rerun-if-env-changed", Value::Singular(var.into()));

        self.custom_instruction(instruction)
    }

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

    pub fn cargo_rustc_flags(&mut self, flags: &str) -> &mut Self {
        let instruction = Instruction::new("rustc-flags", Value::Singular(flags.into()));

        self.custom_instruction(instruction)
    }

    pub fn cargo_rustc_cfg(&mut self, key: &str, value: Option<&str>) -> &mut Self {
        let instruction = Instruction::new(
            "rustc-flags",
            Value::OptionalValue(key.into(), value.map(Into::into)),
        );

        self.custom_instruction(instruction)
    }

    pub fn cargo_rustc_env(&mut self, var: &str, value: &str) -> &mut Self {
        let instruction = Instruction::new(
            "rustc-env",
            Value::UnquotedMapping(var.into(), value.into()),
        );

        self.custom_instruction(instruction)
    }

    pub fn cargo_rustc_cdylib_link_arg(&mut self, flag: &str) -> &mut Self {
        let instruction = Instruction::new("rustc-cdylib-link-arg", Value::Singular(flag.into()));

        self.custom_instruction(instruction)
    }

    pub fn cargo_warning(&mut self, message: &str) -> &mut Self {
        let instruction = Instruction::new("warning", Value::Singular(message.into()));

        self.custom_instruction(instruction)
    }

    pub fn cargo_mapping(&mut self, key: &str, value: &str) -> &mut Self {
        let instruction =
            Instruction::new_mapping(Value::UnquotedMapping(key.into(), value.into()));

        self.custom_instruction(instruction)
    }

    pub fn custom_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.parse_instruction(instruction);

        self
    }
}
