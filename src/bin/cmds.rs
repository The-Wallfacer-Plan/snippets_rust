use std::process::{Command, Stdio};
use std::fs::File;
use std::path::Path;

fn main() {
    let fpath = Path::new("tests/input");

    let f: File = File::open(fpath).expect("cannot open file");

    let output = Command::new("bc").stdin(f).output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();
    println!("{}", res);
}
