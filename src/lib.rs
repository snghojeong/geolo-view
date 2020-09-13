use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(filename: String) -> PyResult<String> {
    read_line(filename)
}

fn read_line(filename: String) -> PyResult<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

