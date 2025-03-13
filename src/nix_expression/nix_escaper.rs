use std::fmt::{self, Write};

use rinja::filters::Escaper;

#[derive(Clone, Copy)]
/// # Nix Escaper
///
/// Escape nix expressions so that they can be written to a template file
pub struct NixEscaper;

impl Escaper for NixEscaper {
    fn write_escaped_str<W: Write>(&self, mut fmt: W, string: &str) -> fmt::Result {
        for character in string.chars() {
            self.write_escaped_char(&mut fmt, character)?
        }

        Ok(())
    }

    fn write_escaped_char<W: Write>(&self, mut fmt: W, c: char) -> fmt::Result {
        match c {
            '"' => fmt.write_str("\\\""),
            '\\' => fmt.write_str("\\\\"),
            '$' => fmt.write_str("\\${}"),
            _ => fmt.write_char(c),
        }
    }
}
