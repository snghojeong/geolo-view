use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

mod log_reader;

fn is_matched(kwds: &Option<Vec<String>>, log_field: &str) -> bool {
    match kwds {
        None => { 
            true
        },
        Some(seq_str) => {
            for s in seq_str {
                if log_field.contains(s) {
                    true
                }
            }
            false
        }
    }
}

fn filter_log<'a>(log_line: &'a String, 
                  seq: &'a Option<Vec<String>>, 
                  date: &'a Option<Vec<String>>, 
                  lv: &'a Option<Vec<String>>, 
                  qlabel: &'a Option<Vec<String>>, 
                  md: &'a Option<Vec<String>>, 
                  msg: &'a Option<Vec<String>>) -> Option<&'a String> {
    let is_match_seq = is_matched(seq, log_reader::seq(log_line.as_str()));
    let is_match_date = is_matched(date, log_reader::date(log_line.as_str()));
    let is_match_time = is_matched(date, log_reader::time(log_line.as_str()));
    let is_match_lv = is_matched(lv, log_reader::level(log_line.as_str()));
    let is_match_qlabel = is_matched(qlabel, log_reader::qlabel(log_line.as_str()));
    let is_match_md = is_matched(md, log_reader::mod_name(log_line.as_str()));
    let is_match_msg = is_matched(msg, log_reader::msg(log_line.as_str()));

    if is_match_seq && is_match_date && is_match_time && is_match_lv && is_match_qlabel && is_match_md && is_match_msg {
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

    let seq = split_filter_keywords(kwds, "seq");
    let date = split_filter_keywords(kwds, "date");
    let lv = split_filter_keywords(kwds, "lv");
    let qlabel = split_filter_keywords(kwds, "qlabel");
    let md = split_filter_keywords(kwds, "md");
    let msg = split_filter_keywords(kwds, "msg");

    let mut pushed_cnt = 0;
    loop {
        let log_line = reader.read_log_line();
        match log_line {
            Ok(unwrap_log_ln) => {
                match filter_log(&unwrap_log_ln, &seq, &date, &lv, &qlabel, &md, &msg) {
                    Some(filtered_log) => {
                        log_buf.push_str(filtered_log.as_str());
                        pushed_cnt += 1;
                    },
                    None => { }
                }
            },
            Err(_) => {
                break;
            }
        }

        if pushed_cnt >= line_cnt {
            break;
        }
    }

    let dict = PyDict::new(py);
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

