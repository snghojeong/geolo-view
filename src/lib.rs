use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::io::SeekFrom;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(path: String, pos: u64, line_cnt: i32, is_reverse: bool) -> PyResult<String> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(pos)).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buffer = String::new();

    buffer.clear();
    let len = reader.read_line(&mut buffer)?;

    Ok(buffer)
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

