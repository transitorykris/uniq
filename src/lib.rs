use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Uniq {
    filename: String,
    case: bool, // true if case sensitive, false if case insensitive
}

impl Uniq {
    // Todo: take a buffer instead of filename
    pub fn from_file(filename: String, case: bool) -> Uniq {
        Uniq { filename, case }
    }

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

    fn write(&mut self, line: String) -> Option<String> {
        if line == self.line {
            return None
        }
        self.line = line.clone();
        Some(line)
    }
}

pub enum UniqErrors {
    NoFile
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        
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
