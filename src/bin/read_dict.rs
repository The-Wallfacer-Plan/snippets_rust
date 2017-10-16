use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Lines, BufReader, BufRead};
use std::path::Path;

const MAX_TOKEN_LENGTH: usize = 16;

macro_rules! panic_line{
    ($line_no:expr, $line:expr) => {{
        panic!("wrong dict format @ [{:04}] --- {}", $line_no, $line);
    }};
}

struct ExtraToken {
    content: Vec<u8>,
}

fn hex_2_byte(raw_byte:u8) -> u8 {
    // '0' to '9'
    if raw_byte > 47 && raw_byte < 58 {
        raw_byte - 48
    }
    // 'A' to 'F'
    else if raw_byte > 64 && raw_byte < 71 {
        // -65 + 10
        raw_byte - 55
    }
    // 'a' to 'f'
    else if raw_byte > 96 && raw_byte < 103 {
        // -97 + 10
        raw_byte - 87
    }
    else {
        panic!("malformed dict hex byte, got ascii: {}", raw_byte);
    }
}

#[inline]
fn read_file_lines<P: AsRef<Path> + Debug>(fpath: P) -> Lines<BufReader<File>> {
    read_file_lines_result(&fpath).unwrap_or_else(|e| panic!("cannot open file {:?}, {:?}", fpath, e))
}

#[inline]
fn read_file_lines_result<P: AsRef<Path> + Debug>(fpath: P) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(&fpath)?;
    let read_buf = BufReader::new(file);
    Ok(read_buf.lines())
}

fn main() {
    let dict_level:u8 = 0;
    let lines = read_file_lines(Path::new("examples/test.dict"));
    'line: for (line_no, line) in lines.enumerate() {
        let l = line.unwrap();

        // skip comments
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }

        let tokens:Vec<&str> = l.split("=").collect();

        let eq_loc = l.find('=');
        if eq_loc == None {
            panic_line!(line_no, l);
        }

        let (key_rs, value_rs) = l.split_at(eq_loc.unwrap());

        let key = key_rs.trim();
        let value_r = value_rs.trim_matches('=').trim();

        if value_r.chars().nth(0) != Some('\"') || value_r.chars().nth(value_r.len()-1) != Some('\"') {
            // println!("first: {}", value_r.chars().nth(0).unwrap());
            // println!("last: {}", value_r.chars().nth(value_r.len()-1).unwrap());
            panic_line!(line_no, l);
        }

        let value = value_r.trim_matches('\"');

        let mut content:Vec<u8> = Vec::with_capacity(MAX_TOKEN_LENGTH);

        if key.contains("@") {
            let key_tokens:Vec<&str> = key.split("@").collect();
            // skip the low level dict tokens
            let key_lvl = key_tokens[1].parse::<u8>().unwrap_or_else(|e| panic!("wrong dict format @ [{}], error {:?}", l, e));
            if key_lvl < dict_level {
                continue;
            }
        }
        else {
            // default key level is 0
            if dict_level > 0 {
                continue;
            }
        }

        if value.contains("\\x") {
            let hex_bytes:Vec<&str> = value.split("\\x").collect();
            let mut counter = 0;
            for hex_byte in hex_bytes {
                let mut raw_byte:u8 = 0;
                for (i, byte) in hex_byte.as_bytes().iter().enumerate() {
                    match i {
                        0 => {
                            raw_byte += hex_2_byte(*byte) * 16;
                        },
                        1 => {
                            if counter >= MAX_TOKEN_LENGTH {
                                continue 'line;
                            }
                            raw_byte += hex_2_byte(*byte);
                            content.push(raw_byte);
                            counter += 1;
                        },
                        _ => {
                            if counter >= MAX_TOKEN_LENGTH {
                                continue 'line;
                            }
                            content.push(*byte);
                            counter += 1;
                        }
                    };
                }
            }
        }
        else {
            if value.as_bytes().len() > MAX_TOKEN_LENGTH {
                continue;
            }
            content.extend(value.as_bytes().iter().cloned());
        }

        println!("key: {}, value: {:?}", key, content);
    }
}