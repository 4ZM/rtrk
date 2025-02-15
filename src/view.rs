use std::time::{Duration, Instant};

use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crossterm::event::KeyEventKind;

use crate::ui::Action;
use crate::ui::ButtonModel;
use crate::ui::RootModel;
use crate::ui::TextBoxModel;

struct TextBoxView {}
impl TextBoxView {
    fn render<W: io::Write>(&self, w: &mut W, m: &TextBoxModel) -> io::Result<()> {
        match m.focus {
            true => queue!(
                w,
                style::Print(format!("[{}]", Self::format_text(&m.text, m.width))),
            )?,
            false => queue!(
                w,
                style::Print(format!("|{}|", Self::format_text(&m.text, m.width))),
            )?,
        }
        Ok(())
    }

    fn format_text(text: &str, width: usize) -> String {
        let blank = width - text.len();
        format!("{}{}", " ".repeat(blank), text)
    }
}

struct ButtonView {}
impl ButtonView {
    fn render<W: io::Write>(&self, w: &mut W, m: &ButtonModel) -> io::Result<()> {
        match m.focus {
            true => queue!(w, style::Print(format!("[{}]", m.text)))?,
            false => queue!(w, style::Print(format!(" {} ", m.text)))?,
        }
        Ok(())
    }
}

struct RootView {
    play_button: ButtonView,
    stop_button: ButtonView,
    scroll_textbox: TextBoxView,
}
impl RootView {
    fn new() -> Self {
        RootView {
            play_button: ButtonView {},
            stop_button: ButtonView {},
            scroll_textbox: TextBoxView {},
        }
    }

    fn render<W: io::Write>(&self, w: &mut W, m: &RootModel) -> io::Result<()> {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        // Would be nice to indicate char with cursor

        queue!(w, cursor::MoveTo(0, 0))?;
        self.play_button.render(w, &m.play_button)?;

        queue!(w, cursor::MoveTo(3, 0))?;
        self.scroll_textbox.render(w, &m.scroll_text_textbox)?;

        queue!(w, cursor::MoveTo(15, 0))?;
        self.stop_button.render(w, &m.stop_button)?;

        w.flush()?;

        Ok(())
    }
}

fn get_event(m: &RootModel) -> Option<Action> {
    if !event::poll(Duration::from_secs(0)).unwrap() {
        return None;
    }

    // Some kind of chain of command pattern here to handle the events in sub views
    // start with the one that has focus, then do the root one
    // might want do do a parent eventually but keep it flat for now

    if m.scroll_text_textbox.focus {
        match event::read() {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::NextFocus),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::SelectCurrent),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::Quit),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => {
                return Some(Action::RmChar(match m.scroll_text_textbox.text.len() {
                    0 => 0,
                    l => l - 1,
                }))
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Delete,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::RmChar(0)),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::EnterChar(c)),
            _ => return None,
        }
    } else {
        match event::read() {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::NextFocus),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::SelectCurrent),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::Play),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::Stop),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::Quit),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Some(Action::Quit),
            _ => return None,
        }
    }
}

pub fn start() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let view = RootView::new();
    let mut m: RootModel = RootModel::new();
    view.render(&mut stdout, &m)?;

    let mut timer = Instant::now();

    loop {
        // Timer event
        let elapsed = timer.elapsed().as_secs_f32();
        if elapsed > 0.1 {
            m.update(Action::Tic);
            timer = Instant::now();
        }

        // User input
        match get_event(&m) {
            Some(Action::Quit) => break,
            Some(a) => m.update(a),
            None => {}
        }

        view.render(&mut stdout, &m)?;
        std::thread::sleep(Duration::from_millis(10));
    }

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}
