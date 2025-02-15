/// The abstract interraction interface between the Application and the UI framework.
/// Different UI frameworks can implement this (e.g. terminal, web, etc).
use crate::pos::Pos;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
    NextFocus,
    PrevFocus,
    Activate,
    Quit,
    Char(char),
    Del,
    DelBack,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    Default,
    Invert,
    Highlight,
}

pub trait Renderer {
    fn clear(&mut self);
    fn render_str(&mut self, _pos: Pos, _text: &str);
    fn render_fmt_str(&mut self, _pos: Pos, _text: &str, _fmt: Style);
    fn flush(&mut self);
}

pub trait EventCollector {
    fn poll_events(&self) -> Vec<crate::interaction::Event>;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub struct TestRenderer {
        pub out: String,
    }
    impl TestRenderer {
        pub fn new() -> Self {
            Self { out: String::new() }
        }
    }
    impl Renderer for TestRenderer {
        fn clear(&mut self) {}
        fn flush(&mut self) {}
        fn render_str(&mut self, _pos: Pos, text: &str) {
            self.out += text;
        }
        fn render_fmt_str(&mut self, _pos: Pos, text: &str, _style: Style) {
            self.out += text;
        }
    }
}
