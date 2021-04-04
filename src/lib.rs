use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use std::str::Split;

mod log_reader;

fn filter_log<'a>(log_line: &'a String, 
                  lv: &'a Option<Vec<String>>, 
                  md: &'a Option<Vec<String>>, 
                  msg: &'a Option<Vec<String>>) -> Option<&'a String> {
    let mut is_match_md = false;
    match (md) {
        None => { 
            is_match_md = true;
        },
        Some(md_str) => {
            for s in md_str {
                if log_reader::mod_name(log_line.as_str()).contains(s) {
                    is_match_md = true;
                }
            }
        }
    }

    let mut is_match_lv = false;
    match (lv) {
        None => { 
            is_match_lv = true;
        },
        Some(lv_str) => {
            for s in lv_str {
                if log_reader::level(log_line.as_str()).contains(s) {
                    is_match_lv = true;
                }
            }
        }
    }

    let mut is_match_msg = false;
    match (msg) {
        None => { 
            is_match_msg = true;
        },
        Some(msg_str) => {
            for s in msg_str {
                if log_reader::msg(log_line.as_str()).contains(s) {
                    is_match_msg = true;
                }
            }
        }
    }

    if is_match_lv && is_match_md && is_match_msg {
        Some(log_line)
    }
    else {
        None
    }
}

fn split_filter_keywords(kwds: Option<&PyDict>, kwd: &str) -> Option<Vec<String>> {
    let item_str: String = kwds?.get_item::<&str>(kwd)?
                           .extract().unwrap_or(None)?;
    let fltr_kwd_list: Vec<&str> = item_str.as_str().split(',').collect();
    let ret_items = fltr_kwd_list.iter().map(|s| { s.to_string() }).collect();
    Some(ret_items)
}

/// Formats the sum of two numbers as string.
#[pyfunction(kwds="**")]
fn read_log(py: Python, path: String, pos: u64, line_cnt: i32, kwds: Option<&PyDict>) -> PyResult<Py<PyDict>> {
    let mut reader = log_reader::LogReader::open(path.as_str(), pos)?;
    let mut log_buf = String::new();

    let md = split_filter_keywords(kwds, "md");
    let lv = split_filter_keywords(kwds, "lv");
    let qlabel = split_filter_keywords(kwds, "qlabel");
    let msg = split_filter_keywords(kwds, "msg");

    let mut pushed_cnt = 0;
    loop {
        let log_line = reader.read_log_line();
        match (log_line) {
            Ok(unwrap_log_ln) => {
                match (filter_log(&unwrap_log_ln, &lv, &md, &msg)) {
                    Some(filtered_log) => {
                        log_buf.push_str(filtered_log.as_str());
                        pushed_cnt += 1;
                    },
                    None => { }
                }
            },
            Err(error) => {
                break;
            }
        }

        if (pushed_cnt >= line_cnt) {
            break;
        }
    }

    let mut dict = PyDict::new(py);
    dict.set_item("pos", reader.strm_pos().unwrap())?;
    dict.set_item("log", log_buf)?;
    Ok(dict.into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn geolo_view(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_log))?;

    Ok(())
}

