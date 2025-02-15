// Messages
enum Message {
    NextFocus,
    SelectFocused,
    Increment,
    Decrement,
    Quit,
}

// The state
#[derive(Default)]
struct Counter {
    value: i64,
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
    fn view(&self) -> Layout {
        let decrement = button("-", Message::Decrement);
        let counter = label(&self.value.to_string());
        let increment = button("+", Message::Increment);

        Layout {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            widgets: vec![decrement, counter, increment],
        }
    }
}

trait Renderable {
    // Doesn't feel right to keep this here - but without it the polymorphism doesn't work
    // since we hold Widgets to draw.
    // Implement it in the UI framework part.
    // Extracting it into it's own trait - does it make it possible to implement in another crate?
    //   yes but there is a dependency on this one to the other crate (not extensible)
    fn render(&self) {}
}

use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

trait Widget: Renderable {
    fn id(&self) -> usize;
    fn children(&self) -> Vec<usize> {
        vec![self.id()]
    }
}

struct Layout {
    id: usize,
    widgets: Vec<Box<dyn Widget>>,
}
impl Widget for Layout {
    fn id(&self) -> usize {
        self.id
    }
    fn children(&self) -> Vec<usize> {
        let mut children: Vec<usize> = vec![self.id()];
        for w in self.widgets.iter() {
            children.append(&mut w.children());
        }
        children
    }
} // To put layouts inside layouts

struct Button {
    id: usize,
    text: String,
    on_press: Message,
}
impl Widget for Button {
    fn id(&self) -> usize {
        self.id
    }
}
fn button(text: &str, on_press: Message) -> Box<Button> {
    Box::new(Button {
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        text: text.to_string(),
        on_press,
    })
}

struct Label {
    id: usize,
    text: String,
}
impl Widget for Label {
    fn id(&self) -> usize {
        self.id
    }
}
fn label(text: &str) -> Box<Label> {
    Box::new(Label {
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        text: text.to_string(),
    })
}

// UI FRAMEWORK STUFF

use crossterm::event::KeyEventKind;
use std::io;
use std::time::Duration;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

// UI Library Interface render and input

fn display(ui: &Layout) {
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

fn interact(_w: &Layout) -> Vec<Message> {
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
            code: KeyCode::Tab,
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) => return vec![Message::NextFocus],
        Ok(Event::Key(KeyEvent {
            code: KeyCode::Enter,
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) => return vec![Message::SelectFocused],
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
impl Renderable for Layout {
    fn render(&self) {
        for w in self.widgets.iter() {
            w.render();
        }
    }
}
impl Renderable for Button {
    fn render(&self) {
        // if self.focus {
        //     execute!(io::stdout(), style::Print(format!("<[{}]>", self.text))).unwrap();
        // } else {
        execute!(io::stdout(), style::Print(format!(" [{}] ", self.text))).unwrap();
        //        }
    }
}

impl Renderable for Label {
    fn render(&self) {
        // if self.focus {
        //     execute!(io::stdout(), style::Print(format!("<({})>", self.text))).unwrap();
        // } else {
        execute!(io::stdout(), style::Print(format!(" ({}) ", self.text))).unwrap();
        //        }
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
