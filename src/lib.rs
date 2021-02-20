use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Result;
use std::str::Split;

fn idx(log_line: &str) -> &str {
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

fn contents(log_line: &str) -> &str {
    &log_line[85..]
}

fn is_log_line(log_line: &str) -> bool {
    if log_line.len() > 4 {
        let log_idx = &log_line[0..3];
        let has_log_idx = if log_idx.parse::<f64>().is_ok() { true }
                          else { false };
        let date = date(log_line);
        let has_date = if date.parse::<f64>().is_ok() { true }
                       else { false };
        if has_log_idx && has_date { true }
        else { false }
    }
    else {
        false
    }
}

fn filter_log<'a>(log_line: &'a String, lv: &'a Option<Vec<&str>>, md: &'a Option<Vec<String>>) -> Option<&'a String> {
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
    fn open(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line_buf = String::new();

        loop {
            reader.read_line(&mut line_buf)?;
            if is_log_line(line_buf.as_str()) {
                break;
            }
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
            self.reader.read_line(&mut self.line_buf)?;
            if is_log_line(self.line_buf.as_str()) {
                break;
            }
        }

        Ok(log_line)
    }
}

fn split_filter_keywords(kwds: Option<&PyDict>, kwd: &str) -> Option<Vec<String>> {
    let dict: &PyDict = kwds?;
    println!("kwds exist!");
    let md_item = dict.get_item::<&str>(kwd)?;
    println!("md item exist!");
    let md_str = md_item.extract();
    let extract_md: Option<String>;
    match (md_str) {
        Err(e) => { return None; },
        Ok(unwrap_md_str) => {
            println!("md str exist!");
            extract_md = unwrap_md_str;
        }
    }
    let split_md = extract_md?;
    println!("split_md exist!");
    let split_md_list: Vec<&str> = split_md.as_str().split(',').collect();

    let mut items = Vec::<String>::with_capacity(split_md_list.len());
    for item in &split_md_list {
        let mut md_str = String::new();
        md_str.push_str(item);
        items.push(md_str);
    }
    return Some(items);
}

/// Formats the sum of two numbers as string.
#[pyfunction(kwds="**")]
fn read_log(path: String, pos: u64, line_cnt: i32, is_backward: bool, kwds: Option<&PyDict>) -> PyResult<String> {
    let mut reader = LogReader::open(path.as_str())?;
    let mut log_buf = String::new();
    let md : Option<String>;
    let lv : Option<String>;
    match (kwds) {
        Some(dict) => {
            let item = dict.get_item::<&str>("md");
            match (item) {
                Some(md_item) => {
                    md = md_item.extract()?;
                },
                None => { md = None; }
            }
            let item = dict.get_item::<&str>("lv");
            match (item) {
                Some(lv_item) => {
                    lv = lv_item.extract()?;
                },
                None => { lv = None; }
            }
        },
        None => { 
            md = None;
            lv = None;
        }
    };

    let split_md = split_filter_keywords(kwds, "md");

    let split_lv = match (&lv) {
        None => { None },
        Some(lv_str) => { Some(lv_str.as_str().split(',').collect()) }
    };

    let mut pushed_cnt = 0;
    loop {
        let log_line = reader.read_log_line()?;
        match (filter_log(&log_line, &split_lv, &split_md)) {
            Some(filtered_log) => {
                log_buf.push_str(filtered_log.as_str());
                pushed_cnt += 1;
            },
            None => { }
        }

        if (pushed_cnt >= line_cnt) {
            break;
        }
    }

    Ok(log_buf)
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

