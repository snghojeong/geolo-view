fn main() {
    let mut log_buf = String::new();

    let date = split_filter_keywords(kwds, "date");
    let time = split_filter_keywords(kwds, "time");
    let lv = split_filter_keywords(kwds, "lv");
    let qlabel = split_filter_keywords(kwds, "qlabel");
    let md = split_filter_keywords(kwds, "md");
    let msg = split_filter_keywords(kwds, "msg");

    let mut pushed_cnt = 0;
    loop {
        let log_line = reader.read_log_line();
        match log_line {
            Ok(unwrap_log_ln) => {
                match filter_log(&unwrap_log_ln, &date, &time, &lv, &qlabel, &md, &msg) {
                    Some(filtered_log) => {
                        log_buf.push_str(filtered_log.as_str());
                        pushed_cnt += 1;
                    },
                    None => { }
                }
            },
            Err(e) => {
                print!("Failed to read log line. Error message: {}", e);
                break;
            }
        }

        if (line_cnt > 0) && (pushed_cnt >= line_cnt) {
            break;
        }
    }

    let dict = PyDict::new(py);
    dict.set_item("pos", reader.strm_pos().unwrap())?;
    dict.set_item("log", log_buf)?;
    Ok(dict.into())
}
