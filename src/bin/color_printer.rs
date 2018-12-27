use term::color::*;
use term::{self, StdoutTerminal};

pub struct ColorPrinter {
    t: Box<StdoutTerminal>,
}

impl ColorPrinter {
    pub fn new() -> ColorPrinter {
        let t = term::stdout().unwrap();
        ColorPrinter { t: t }
    }
    pub fn print(&mut self, color: Color, s: &str) {
        self.t.fg(color).unwrap();
        write!(self.t, "{}", s).unwrap();
    }

    pub fn println(&mut self, color: Color, s: &str) {
        self.t.fg(color).unwrap();
        write!(self.t, "{}", s).unwrap();
    }
}

impl Default for ColorPrinter {
    fn default() -> Self {
        ColorPrinter::new()
    }
}
