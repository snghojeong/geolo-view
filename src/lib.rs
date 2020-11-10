use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(path: String, line_ofs: i32, line_cnt: i32) -> PyResult<String> {
    let file = File::open(path)?;
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

