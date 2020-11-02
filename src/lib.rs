use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(filename: String, line_ofs: i32, line_cnt: i32) -> PyResult<String> {
    let mut file = File::open(filename)?;
    read_line(file, 0)
}

fn read_line(file: File, idx: i32) -> PyResult<String> {
    Ok(io::BufReader::new(file).lines()[idx])
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

