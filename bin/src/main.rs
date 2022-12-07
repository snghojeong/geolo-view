mod log_reader;

fn main() {
    let mut log_buf = String::new();
    let mut pushed_cnt = 0;
    loop {
        let log_line = reader.read_log_line();
        match log_line {
            Ok(unwrap_log_ln) => {
                log_buf.push_str(unwrap_log_ln);
                pushed_cnt += 1;
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
    
    print!(log_buf);
}
