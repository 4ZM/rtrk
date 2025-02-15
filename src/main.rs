use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crossterm::event::KeyEventKind;

struct Model {
    playing: bool,
}
impl Model {
    fn new() -> Self {
        Model { playing: false }
    }
}

// Should be member of the model?
fn update(a: Action, m: &mut Model) {
    match a {
        Action::Play => m.playing = true,
        Action::Stop => m.playing = false,
        _ => {}
    };
}

#[derive(Debug)]
enum Action {
    Play,
    Stop,
    Quit,
}

struct View {}
impl View {}

fn render<W: io::Write>(w: &mut W, m: &Model) -> io::Result<()> {
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    if m.playing {
        queue!(w, style::Print("PLAYING"))?;
    } else {
        queue!(w, style::Print("STOPPED"))?;
    }

    w.flush()?;

    Ok(())
}

fn get_event() -> Action {
    loop {
        match dbg!(event::read()) {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Action::Play,
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Action::Stop,
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return Action::Quit,
            _ => {}
        }
    }
}

fn start() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let mut m: Model = Model::new();
    render(&mut stdout, &m)?;

    loop {
        match get_event() {
            Action::Quit => break,
            a => update(a, &mut m),
        }
        render(&mut stdout, &m)?;
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

fn main() {
    start().expect("BOOM");
}
