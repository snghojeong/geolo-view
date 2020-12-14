use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Result;

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

fn read_log_line(reader: &mut dyn BufRead) -> Result<String> {
    let mut buffer = String::new();
    let mut ret = String::new();

    loop {
        buffer.clear();
        reader.read_line(&mut buffer)?;

        if is_log_line(buffer.as_str()) {
            if ret.len() > 0 {
                break;
            }
            else {
                ret.push_str(buffer.as_str());
            }
        }
        else {
            ret.push_str(buffer.as_str());
        }
    }

    Ok(ret)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(path: String, pos: u64, line_cnt: i32, is_reverse: bool) -> PyResult<String> {
    let mut file = File::open(path)?;
    if is_reverse {
        file.seek(SeekFrom::Start(pos-100)).unwrap();
    } else {
        file.seek(SeekFrom::Start(pos)).unwrap();
    }
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    buffer.clear();
    for _n in 0..line_cnt {
        let log_line_str = read_log_line(&mut reader)?;
        buffer.push_str(&log_line_str);
    }

    Ok(buffer)
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

