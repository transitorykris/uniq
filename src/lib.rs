use std::fs::File;
use std::io::{BufReader, BufRead};

// TODO: Add docstrings

pub struct Uniq {
    filename: String,   // XXX use Path here instead of String for safety

    // TODO: complete case sensitivity feeature
    case: bool, // true if case sensitive, false if case insensitive
}

impl Uniq {
    pub fn from_file(filename: String, case: bool) -> Uniq {
        // TODO: open the file here and store a buffer in struct Uniq
        Uniq { filename, case }
    }

    // TODO: Refactor to take in a generic buffer, move file handling elsewhere
    // Consider refactoring into an iterator to increase testability
    pub fn run(&self) -> Result<(), UniqErrors> {
        let text_file = File::open(&self.filename); // XXX a buffer of some kind
        let text_raw = match text_file {
            Ok(t) => t,
            Err(_) => return Err(UniqErrors::NoFile),
        };
        let reader = BufReader::new(text_raw);
        
        let mut line_buf = LineBuffer::new();
    
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    match line_buf.write(line) {
                        // TODO: refactor into a writer that we can test
                        Some(line) => println!("{}", line), // XXX side effect
                        None => {},
                    }
                },
                Err(e) => println!("err: {}", e),
            }
        }
        Ok(())
    }
}

struct LineBuffer {
    line: String,
}

impl LineBuffer {
    fn new() -> LineBuffer {
        LineBuffer{line: String::new()}
    }

    // XXX this is not actually a 'writer', rename to something idomatic
    fn write(&mut self, line: String) -> Option<String> {
        if line == self.line {
            return None
        }
        self.line = line.clone();
        Some(line)
    }
}

// TODO: Define values for the error codes to match Linux error codes
pub enum UniqErrors {
    NoFile
}

#[cfg(test)]
mod tests {
    // TODO: Add a test for Uniq::from_file

    #[test]
    fn run() {
        // TODO test me!
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
            None => panic!("bad line"),
        }
        match line_buf.write(lines[1].to_string()) {
            Some(l) => assert_eq!(l, lines[1]),
            None => panic!("bad line"),
        }
        match line_buf.write(lines[2].to_string()) {
            Some(l) => assert_ne!(l, lines[2]),
            None => assert!(true),
        }
        match line_buf.write(lines[3].to_string()) {
            Some(l) => assert_eq!(l, lines[3]),
            None => panic!("bad line"),
        }
    }
}
