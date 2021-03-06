use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Result;
use std::str::Split;

fn seq(log_line: &str) -> &str {
    &log_line[0..3]
}

fn date(log_line: &str) -> &str {
    &log_line[5..11]
}

fn level(log_line: &str) -> &str {
    &log_line[25..29]
}

fn qlabel(log_line: &str) -> &str {
    &log_line[30..57]
}

fn mod_name(log_line: &str) -> &str {
    &log_line[58..78]
}

fn line(log_line: &str) -> &str {
    &log_line[80..83]
}

fn msg(log_line: &str) -> &str {
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

fn filter_log<'a>(log_line: &'a String, 
                  lv: &'a Option<Vec<String>>, 
                  md: &'a Option<Vec<String>>, 
                  msg: &'a Option<Vec<String>>) -> Option<&'a String> {
    let mut is_match_md = false;
    match (md) {
        None => { 
            is_match_md = true;
        },
        Some(md_str) => {
            for s in md_str {
                if mod_name(log_line.as_str()).contains(s) {
                    is_match_md = true;
                }
            }
        }
    }

    let mut is_match_lv = false;
    match (lv) {
        None => { 
            is_match_lv = true;
        },
        Some(lv_str) => {
            for s in lv_str {
                if level(log_line.as_str()).contains(s) {
                    is_match_lv = true;
                }
            }
        }
    }

    if is_match_lv && is_match_md {
        Some(log_line)
    }
    else {
        None
    }
}

struct LogReader {
    reader: BufReader<File>,
    line_buf: String,
}

impl LogReader {
    fn open(path: &str, pos: u64) -> Result<Self> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(pos)).unwrap();
        let mut reader = BufReader::new(file);
        let mut line_buf = String::new();

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

    fn read_log_line(&mut self) -> Result<String> {
        let mut log_line = String::new();

        loop {
            log_line.push_str(self.line_buf.as_str());
            self.line_buf.clear();
            let read_len = self.reader.read_line(&mut self.line_buf)?;
            if read_len == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
            }
            else if is_log_line(self.line_buf.as_str()) {
                break;
            }
        }

        Ok(log_line)
    }

    fn strm_pos(&mut self) -> Result<u64> {
        self.reader.seek(SeekFrom::Current(0))
    }
}

fn split_filter_keywords(kwds: Option<&PyDict>, kwd: &str) -> Option<Vec<String>> {
    let item_str: String = kwds?.get_item::<&str>(kwd)?
                           .extract().unwrap_or(None)?;
    let fltr_kwd_list: Vec<&str> = item_str.as_str().split(',').collect();
    let ret_items = fltr_kwd_list.iter().map(|s| { s.to_string() }).collect();
    Some(ret_items)
}

/// Formats the sum of two numbers as string.
#[pyfunction(kwds="**")]
fn read_log(py: Python, path: String, pos: u64, line_cnt: i32, is_backward: bool, kwds: Option<&PyDict>) -> PyResult<Py<PyDict>> {
    let mut reader = LogReader::open(path.as_str(), pos)?;
    let mut log_buf = String::new();

    let md = split_filter_keywords(kwds, "md");
    let lv = split_filter_keywords(kwds, "lv");
    let msg = split_filter_keywords(kwds, "msg");

    let mut pushed_cnt = 0;
    loop {
        let log_line = reader.read_log_line();
        match (log_line) {
            Ok(unwrap_log_ln) => {
                match (filter_log(&unwrap_log_ln, &lv, &md, &msg)) {
                    Some(filtered_log) => {
                        log_buf.push_str(filtered_log.as_str());
                        pushed_cnt += 1;
                    },
                    None => { }
                }
            },
            Err(error) => {
                break;
            }
        }

        if (pushed_cnt >= line_cnt) {
            break;
        }
    }

    let mut dict = PyDict::new(py);
    dict.set_item("pos", reader.strm_pos().unwrap())?;
    dict.set_item("log", log_buf)?;
    Ok(dict.into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

