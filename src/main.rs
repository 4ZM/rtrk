use std::time::Duration;

use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crossterm::event::KeyEventKind;

// The state

#[derive(Default)]
struct Counter {
    value: i64,
}

// Messages
enum Message {
    Increment,
    Decrement,
    Quit,
}

impl Counter {
    // Update state (self)
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            _ => (),
        }
    }

    // Build widget tree given state (self)
    fn view(&self) -> Ui {
        let decrement = Button {
            text: "-".to_string(),
            on_press: Message::Decrement,
        };
        let counter = Label {
            text: self.value.to_string(),
        };
        let increment = Button {
            text: "+".to_string(),
            on_press: Message::Increment,
        };

        Ui {
            widgets: vec![Box::new(decrement), Box::new(counter), Box::new(increment)],
        }
    }
}

// UI Library Interface render and input

fn display(ui: &Ui) {
    let _ = execute!(
        std::io::stdout(),
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    );

    for widget in ui.widgets.iter() {
        widget.render();
    }
}

fn interact(_w: &Ui) -> Vec<Message> {
    if !event::poll(Duration::from_secs(0)).unwrap() {
        return vec![];
    }

    match event::read() {
        Ok(Event::Key(KeyEvent {
            code: KeyCode::Esc,
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) => return vec![Message::Quit],
        Ok(Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) => return vec![Message::Decrement],
        Ok(Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) => return vec![Message::Increment],
        _ => vec![],
    }
}

// Widgets

trait Widget {
    fn render(&self);
}

struct Ui {
    widgets: Vec<Box<dyn Widget>>,
}
//impl Widget for Pane {}

struct Button {
    text: String,
    on_press: Message,
}
impl Widget for Button {
    fn render(&self) {
        execute!(io::stdout(), style::Print(format!("[{}]", self.text))).unwrap();
    }
}

struct Label {
    text: String,
}
impl Widget for Label {
    fn render(&self) {
        execute!(io::stdout(), style::Print(format!("({})", self.text))).unwrap();
    }
}

// Runtime
fn start() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(&mut stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut state = Counter::default();
    'app: loop {
        let ui = state.view();
        display(&ui);

        let messages = interact(&ui);

        for message in messages {
            match message {
                Message::Quit => break 'app,
                _ => state.update(message),
            }
        }

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
    start().expect("UI Failed Unexpectedly");
}
