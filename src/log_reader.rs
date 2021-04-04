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
    reader: BufReader<File>,
    line_buf: String,
}

impl LogReader {
    pub fn open(path: &str, pos: u64) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line_buf = String::new();

        reader.seek(SeekFrom::Start(pos)).unwrap();

        loop {
            reader.read_line(&mut line_buf)?;
            if is_log_line(line_buf.as_str()) {
                break;
            }
            line_buf.clear();
        }

        let mut inst = LogReader {
            reader: reader,
            line_buf: line_buf,
        };

        Ok(inst)
    }

    pub fn read_log_line(&mut self) -> Result<String> {
        let mut log_line = String::new();

        loop {
            log_line.push_str(self.line_buf.as_str());
            self.line_buf.clear();
            self.reader.read_line(&mut self.line_buf)?;
            if is_log_line(self.line_buf.as_str()) {
                break;
            }
        }

        Ok(log_line)
    }

    pub fn strm_pos(&mut self) -> Result<u64> {
        self.reader.seek(SeekFrom::Current(0))
    }
}

