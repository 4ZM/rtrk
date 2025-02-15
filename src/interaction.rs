/// The abstract interraction interface between the Application and the UI framework.
/// Different UI frameworks can implement this (e.g. terminal, web, etc).
use crate::pos::Pos;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
    Next,
    //Back, // TODO Add back support
    Activate,
    Quit,
    Char(char),
}

pub trait Renderer {
    fn clear(&mut self);
    fn render_str(&mut self, _pos: Pos, _text: &str);
    fn flush(&mut self);
}

pub trait EventCollector {
    fn poll_events(&self) -> Vec<crate::interaction::Event>;
}
