use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Result;

fn date(log_line: &str) -> &str {
    &log_line[5..10]
}

fn level(log_line: &str) -> &str {
    &log_line[25..28]
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
        let date = &log_line[5..10];
        let has_date = if date.parse::<f64>().is_ok() { true }
                       else { false };
        if has_log_idx && has_date { true }
        else { false }
    }
    else {
        false
    }
}

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

    log_buf.clear();
    for _n in 0..line_cnt {
        let mut line_buf = String::new();
        loop {
            line_buf.clear();
            reader.read_line(&mut line_buf)?;

            log_buf.push_str(line_buf.as_str());
            if is_log_line(line_buf.as_str()) {
                break;
            }
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

