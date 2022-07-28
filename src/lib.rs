use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// TODO: Add docstrings

pub enum UniqErrors {
    NoFile,
    ReadError,
    WriteError,
}

pub struct Uniq {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,

    pub ignore_case: bool,
    pub count: bool,
}

impl Default for Uniq {
    fn default() -> Uniq {
        Uniq {
            reader: Box::new(BufReader::new(std::io::stdin())),
            writer: Box::new(BufWriter::new(std::io::stdout())),
            ignore_case: false,
            count: false,
        }
    }
}

impl Uniq {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Uniq, UniqErrors> {
        let mut u: Uniq = Uniq::new();
        match File::open(filename) {
            Ok(t) => {
                u.reader = Box::new(BufReader::new(t));
            }
            Err(_) => return Err(UniqErrors::NoFile),
        };
        Ok(u)
    }

    // TODO: Consider refactoring into an iterator to increase testability
    pub fn run(&mut self) -> Result<(), UniqErrors> {
        let mut prev_line = String::new();
        let mut count = 0;
        loop {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => return Ok(()), // Got 0 bytes, EOF
                Ok(_) => {
                    count += 1;
                    // we need to suppress printing the first line we receive
                    if prev_line.is_empty() { prev_line = line.clone(); }

                    if (line == prev_line) ||
                        self.ignore_case && line.to_lowercase() == prev_line.to_lowercase() {
                        continue
                    }

                    // we have a new uniq line, write out the previously repeated line
                    if self.count {
                        Self::write(self, &Uniq::format_count(count, &prev_line))?;
                    } else {
                        Self::write(self, &prev_line)?;
                    }

                    prev_line = line;
                    count = 0;
                },
                Err(_) => return Err(UniqErrors::ReadError),
            }
        }
    }

    fn format_count(count: i32, string: &str) -> String {
        let mut out = String::new();
        out.push_str("   ");
        out.push_str(&count.to_string());
        out.push(' ');
        out.push_str(&string);
        out.to_string()
    }

    fn write(&mut self, line: &str) -> Result<(), UniqErrors> {
        match write!(self.writer, "{}", line) {
            Ok(_) => {self.writer.flush().unwrap()},    // safe to unwrap
            Err(_) => return Err(UniqErrors::WriteError),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default() {
        let u = super::Uniq::default();
        assert!(!u.ignore_case);
        assert!(!u.count);
    }

    #[test]
    fn new() {
        let u = super::Uniq::default();
        assert!(!u.ignore_case);
        assert!(!u.count);
    }

    #[test]
    fn from_file() {
        let u = super::Uniq::from_file("test/testlines");
        match u {
            Ok(mut u) => {
                let mut l = String::new();
                assert_ne!(u.reader.read_line(&mut l).unwrap(), 0);
            }
            Err(_) => {
                panic!("unexpected error opening file");
            }
        };
    }

    #[test]
    fn run() {
        use std::io::{BufReader, BufWriter, Cursor};
        let line_cursor = Cursor::new("a\nb\nb\nc");
        let mut u = super::Uniq::new();
        u.reader = Box::new(BufReader::new(line_cursor));
        let writer = BufWriter::new(Vec::new());
        u.writer = Box::new(writer);
        match u.run() {
            Ok(_) => {}
            Err(_) => panic!("run should not have returned error"),
        }
        // TODO: check what was written
        // TODO: add a test for case sensitivity
    }
}
