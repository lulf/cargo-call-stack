use std::{fmt, io};

pub(crate) struct Escaper<W>
where
    W: io::Write,
{
    pub(crate) writer: W,
    pub(crate) error: io::Result<()>,
}

impl<W> Escaper<W>
where
    W: io::Write,
{
    pub(crate) fn new(writer: W) -> Self {
        Escaper {
            writer,
            error: Ok(()),
        }
    }
}

impl<W> fmt::Write for Escaper<W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        match (|| -> io::Result<()> {
            match c {
                '"' => write!(self.writer, "\\")?,
                _ => {}
            }

            write!(self.writer, "{}", c)
        })() {
            Err(e) => {
                self.error = Err(e);

                Err(fmt::Error)
            }
            Ok(()) => Ok(()),
        }
    }
}
