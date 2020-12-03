use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::Result;

fn read_log_line(reader: &mut dyn BufRead) -> Result<String> {
    let mut buffer = String::new();
    buffer.clear();
    reader.read_line(&mut buffer)?;
    Ok(buffer)
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
        buffer.push_str(&*read_log_line(&mut reader)?);
    }

    Ok(buffer)
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

