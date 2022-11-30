use std::fs::File;
use std::io::Result;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::prelude::*;
use chrono::NaiveTime;
use chrono::format::ParseResult;

pub fn seq(log_line: &str) -> &str {
    &log_line[0..3]
}

pub fn date(log_line: &str) -> &str {
    &log_line[5..10]
}

pub fn time(log_line: &str) -> ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(&log_line[12..23], "%H:%M:%S")
}

pub fn level(log_line: &str) -> &str {
    &log_line[25..29]
}

pub fn qlabel(log_line: &str) -> &str {
    &log_line[30..55]
}

pub fn mod_name(log_line: &str) -> &str {
    &log_line[57..77]
}

pub fn msg(log_line: &str) -> &str {
    &log_line[84..]
}

fn is_log_line(log_line: &str) -> bool {
    if log_line.len() > 85 {
        let log_seq = seq(log_line);
        let log_date = date(log_line);
        let has_log_seq = if log_seq.parse::<f64>().is_ok() { true }
                          else { false };
        let has_log_date = if log_date.parse::<f64>().is_ok() { true }
                           else { false };
        if has_log_seq && has_log_date { true }
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
        let file = File::open(path)?;
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

        let inst = LogReader {
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
            let read_len = self.reader.read_line(&mut self.line_buf)?;
            if read_len == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "End of file"));
            }
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

