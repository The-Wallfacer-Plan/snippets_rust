#[derive(Fail, Debug)]
#[fail(display = "Input was invalid UTF-8 at index {}", index)]
pub struct Utf8Error {
    index: usize,
}
