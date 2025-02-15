use std::time::{Duration, Instant};

use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crossterm::event::KeyEventKind;

enum Action {
    Tic,
    NextFocus,
    SelectCurrent,
    Play,
    Stop,
    Quit,
}

#[derive(Clone)]
enum Widget {
    StopButton,
    PlayButton,
}

struct Model {
    playing: bool,
    text: String,
    focus: Widget,
}
impl Model {
    fn update(&mut self, a: Action) {
        match a {
            Action::Play => self.playing = true,
            Action::Stop => self.playing = false,
            Action::NextFocus => self.focus = Model::next_focus(self.focus.clone()),
            Action::SelectCurrent => match self.focus {
                Widget::PlayButton => self.playing = true,
                Widget::StopButton => self.playing = false,
            },
            Action::Tic => {
                if self.playing {
                    self.scroll_text()
                }
            }
            _ => {}
        };
    }

    fn scroll_text(&mut self) {
        if let Some(c) = self.text.pop() {
            self.text.insert(0, c);
        }
    }

    fn next_focus(w: Widget) -> Widget {
        match w {
            Widget::PlayButton => Widget::StopButton,
            Widget::StopButton => Widget::PlayButton,
        }
    }

    fn new() -> Self {
        Model {
            text: ".-''-._".to_string(),
            playing: true,
            focus: Widget::PlayButton,
        }
    }
}

struct View {}
impl View {
    fn render<W: io::Write>(&self, w: &mut W, m: &Model) -> io::Result<()> {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        match m.focus {
            Widget::PlayButton => queue!(w, style::Print("[>]"))?,
            Widget::StopButton => queue!(w, style::Print(" > "))?,
        }

        queue!(w, style::Print(format!(" |{}| ", m.text)))?;

        match m.focus {
            Widget::PlayButton => queue!(w, style::Print(" . "))?,
            Widget::StopButton => queue!(w, style::Print("[.]"))?,
        }

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

fn start() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let view = View {};
    let mut m: Model = Model::new();
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

fn main() {
    start().expect("BOOM");
}
