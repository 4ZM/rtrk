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
}
impl RootView {
    fn new() -> Self {
        RootView {
            play_button: ButtonView {},
            stop_button: ButtonView {},
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

        self.play_button.render(w, &m.play_button)?;

        queue!(w, style::Print(format!(" |{}| ", m.text)))?;

        self.stop_button.render(w, &m.stop_button)?;

        w.flush()?;

        Ok(())
    }
}

fn get_event() -> Option<Action> {
    if !event::poll(Duration::from_secs(0)).unwrap() {
        return None;
    }

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
        _ => return None,
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
        match get_event() {
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
