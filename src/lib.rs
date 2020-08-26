use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_log(filename: String) -> PyResult<String> {
    Ok(filename)
}

fn read_line(filename: String) -> String {
    "Line"
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

