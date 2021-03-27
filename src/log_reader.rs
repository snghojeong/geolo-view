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
    is_backward: bool,
    buf_reader: Option<BufReader<File>>,
    rev_lines: Option<RevLines<File>>,
    line_buf: String,
}

impl LogReader {
    fn read_line(&mut self) -> Result<&String> {
        if self.is_backward {
            let rev_line_result = self.rev_lines.as_mut().unwrap().next();
            match (rev_line_result) {
                Some(line_str) => { 
                    self.line_buf = line_str;
                    Ok(&self.line_buf) 
                },
                None => Err(std::io::Error::new(std::io::ErrorKind::Other, "")),
            }
        }
        else {
            self.buf_reader.as_mut().unwrap().read_line(&mut self.line_buf)?;
            Ok(&self.line_buf)
        }
    }

    pub fn open(path: &str, pos: u64, is_backward: bool) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line_buf = String::new();

        let mut inst = LogReader {
            is_backward: is_backward,
            buf_reader: None,
            rev_lines: None,
            line_buf: line_buf,
        };

        if is_backward {
            let mut rev_lines = RevLines::new(reader, pos).unwrap();
            inst.rev_lines = Some(rev_lines);
        }
        else {
            reader.seek(SeekFrom::Start(pos)).unwrap();
            inst.buf_reader = Some(reader);
        }

        loop {
            let line_str = inst.read_line()?;
            if is_log_line(line_str.as_str()) {
                break;
            }
            inst.line_buf.clear();
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
        if self.is_backward {
            Ok(0)
        }
        else {
            let buf_reader = self.buf_reader.as_mut().unwrap();
            buf_reader.seek(SeekFrom::Current(0))
        }
    }
}

