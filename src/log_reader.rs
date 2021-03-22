use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use rev_lines::RevLines;

pub fn seq(log_line: &str) -> &str {
    &log_line[0..3]
}

pub fn date(log_line: &str) -> &str {
    &log_line[5..11]
}

pub fn level(log_line: &str) -> &str {
    &log_line[25..29]
}

pub fn qlabel(log_line: &str) -> &str {
    &log_line[30..57]
}

pub fn mod_name(log_line: &str) -> &str {
    &log_line[58..78]
}

pub fn line(log_line: &str) -> &str {
    &log_line[80..83]
}

pub fn msg(log_line: &str) -> &str {
    &log_line[85..]
}

enum Direction {
    Forward(BufReader<File>),
    Backward(RevLines<File>),
}

fn is_log_line(log_line: &str) -> bool {
    if log_line.len() > 4 {
        let log_seq = &log_line[0..3];
        let has_log_seq = if log_seq.parse::<f64>().is_ok() { true }
                          else { false };
        let date = date(log_line);
        let has_date = if date.parse::<f64>().is_ok() { true }
                       else { false };
        if has_log_seq && has_date { true }
        else { false }
    }
    else {
        false
    }
}

pub struct LogReader {
    direction: Direction,
    line_buf: String,
}

impl LogReader {
    fn read_line(&mut self) -> Result<String> {
        match(&(self.direction)) {
            Direction::Forward(buf_reader) => {
                buf_reader.read_line(&mut self.line_buf)?;
                Ok(self.line_buf)
            },
            Direction::Backward(rev_lines) => {
                let rev_line_result = rev_lines.next();
                match (rev_line_result) {
                    Some(line_str) => { 
                        self.line_buf = line_str;
                        Ok(self.line_buf) 
                    },
                    None => Err(std::io::Error::new(std::io::ErrorKind::Other, "")),
                }
            }
        }
    }

    pub fn open(path: &str, pos: u64, is_backward: bool) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line_buf = String::new();

        let inst = LogReader {
            direction: if is_backward { 
                           let mut rev_lines = RevLines::new(reader, pos).unwrap();
                           Direction::Backward(rev_lines) 
                       } 
                       else { 
                           reader.seek(SeekFrom::Start(pos)).unwrap();
                           Direction::Forward(reader) },
            line_buf: line_buf,
        };

        loop {
            let line_str = inst.read_line()?;
            if is_log_line(line_str.as_str()) {
                break;
            }
            line_buf.clear();
        }

        Ok(inst)
    }

    pub fn read_log_line(&mut self) -> Result<String> {
        let mut log_line = String::new();

        loop {
            log_line.push_str(self.line_buf.as_str());
            self.line_buf.clear();
            let read_ret = self.read_line()?;
            if is_log_line(self.line_buf.as_str()) {
                break;
            }
        }

        Ok(log_line)
    }

    pub fn strm_pos(&mut self) -> Result<u64> {
        match(self.direction) {
            Direction::Forward(buf_reader) => {
                buf_reader.seek(SeekFrom::Current(0))
            },
            Direction::Backward(rev_lines) => {
                Ok(0)
            }
        }
    }
}

