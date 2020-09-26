use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(filename: String) -> PyResult<String> {
    read_line(filename, 0)
}

fn read_line(filename: String, idx: i32) -> PyResult<String> {
    let mut file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()[idx])
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

