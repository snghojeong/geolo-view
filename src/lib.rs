use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Result;

fn idx(log_line: &str) -> &str {
    &log_line[0..3]
}

fn date(log_line: &str) -> &str {
    &log_line[5..10]
}

fn level(log_line: &str) -> &str {
    &log_line[25..29]
}

fn mod_name(log_line: &str) -> &str {
    &log_line[30..28]
}

fn contents(log_line: &str) -> &str {
    &log_line[29..]
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

/*
fn filter_log(log_line: &str, lv: &str, md: &str) -> Option<&str> {
    if level(log_line).trim() == lv {
        Some(log_line)
    }
    else {
        None
    }
}
*/

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(path: String, pos: u64, line_cnt: i32, lv: String, md: String, is_reverse: bool) -> PyResult<String> {
    let mut file = File::open(path)?;
    if is_reverse {
        file.seek(SeekFrom::Start(pos-100)).unwrap();
    } else {
        file.seek(SeekFrom::Start(pos)).unwrap();
    }
    let mut reader = BufReader::new(file);
    let mut log_buf = String::new();

    let mut line_buf = String::new();
    let mut pushed_cnt = 0;
    reader.read_line(&mut line_buf)?;
    loop {

        if is_log_line(line_buf.as_str()) && (level(line_buf.as_str()).trim() == lv) {
            log_buf.push_str(line_buf.as_str());

            loop {
                line_buf.clear();
                reader.read_line(&mut line_buf)?;

                if is_log_line(line_buf.as_str()) {
                    pushed_cnt += 1;
                    break;
                }
                else {
                    log_buf.push_str(line_buf.as_str());
                }
            }
        }
        else {
            line_buf.clear();
            reader.read_line(&mut line_buf)?;
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

