use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// TODO: Add docstrings

// TODO: Define values for the error codes to match Linux error codes
pub enum UniqErrors {
    NoFile,
    ReadError,
    WriteError,
}

pub struct Uniq {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,

    // TODO: complete case sensitivity feeature
    #[allow(dead_code)]
    case: bool, // true if case sensitive, false if case insensitive
}

impl Default for Uniq {
    fn default() -> Uniq {
        Uniq {
            reader: Box::new(BufReader::new(std::io::stdin())),
            writer: Box::new(BufWriter::new(std::io::stdout())),
            case: true,
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

    // Consider refactoring into an iterator to increase testability
    pub fn run(&mut self) -> Result<(), UniqErrors> {
        let mut line_buf = LineBuffer::new();

        loop {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => return Ok(()), // Got 0 bytes, EOF
                Ok(_) => {
                    if let Some(l) = line_buf.write(line) {
                        match write!(self.writer, "{}", l) {
                            Ok(_) => {}
                            Err(_) => return Err(UniqErrors::WriteError),
                        };
                    }
                }
                Err(_) => return Err(UniqErrors::ReadError),
            }
        }
    }
}

struct LineBuffer {
    line: String,
}

impl LineBuffer {
    fn new() -> LineBuffer {
        LineBuffer {
            line: String::new(),
        }
    }

    // XXX this is not actually a 'writer', rename to something idomatic
    fn write(&mut self, line: String) -> Option<String> {
        if line == self.line {
            return None;
        }
        self.line = line.clone();
        Some(line)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default() {
        let u = super::Uniq::default();
        assert!(u.case);
    }

    #[test]
    fn new() {
        let u = super::Uniq::default();
        assert!(u.case);
    }

    #[test]
    fn from_file() {
        //todo!();
    }

    #[test]
    fn run() {
        //todo!();
    }

    #[test]
    fn linebuffer() {
        let lines = vec![
            "With some duplicate lines",
            "here is one",
            "here is one",
            "This is not one",
        ];
        let mut line_buf = super::LineBuffer::new();
        match line_buf.write(lines[0].to_string()) {
            Some(l) => assert_eq!(l, lines[0]),
            None => panic!("unexpected None"),
        }
        match line_buf.write(lines[1].to_string()) {
            Some(l) => assert_eq!(l, lines[1]),
            None => panic!("unexpected None"),
        }
        match line_buf.write(lines[2].to_string()) {
            Some(l) => assert_ne!(l, lines[2]),
            None => {},
        }
        match line_buf.write(lines[3].to_string()) {
            Some(l) => assert_eq!(l, lines[3]),
            None => panic!("unexpected None"),
        }
    }
}
