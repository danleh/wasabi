use std::fmt;

/// Wrapper around a fmt::Write that indents every line by two spaces.
/// The current line is not indented (since it is already written out to the underlying writer),
/// so make sure to write a '\n' after calling fmt::Writer.indent().
pub struct IndentWrite<'w>(&'w mut fmt::Write);

impl<'w> fmt::Write for IndentWrite<'w> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let indented_lines = s.replace('\n', "\n  ");
        self.0.write_str(&indented_lines)
    }
}

/// Extension trait to wrap self of type fmt::Write into an IndentationWrite.
pub trait IndentExt {
    fn indent(&mut self) -> IndentWrite;
}

impl<W: fmt::Write> IndentExt for W {
    fn indent(&mut self) -> IndentWrite {
        IndentWrite(self)
    }
}
