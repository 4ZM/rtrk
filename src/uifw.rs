pub mod interaction;
pub mod pos;
mod term;
pub mod widget;

use crate::app;
use interaction::{EventCollector, Renderer};
use pos::Pos;
use std::collections::VecDeque;
use std::time::Duration;
use widget::{View, Widget};

// Runtime - TODO Generalize on task
pub fn start<Message, V: View<Message>>(app: &mut dyn Widget<Message, app::Task, V>) {
    let mut renderer = term::CrosstermRenderer::new(std::io::stdout());
    let event_collector = term::CrosstermEventCollector {};

    'app: loop {
        // Render state
        let view = app.view(Pos { r: 0, c: 0 });
        renderer.clear();
        view.draw(&mut renderer);
        renderer.flush();

        std::thread::sleep(Duration::from_millis(30));

        // Get UI event interactions
        let mut unprocessed_messages = VecDeque::<Message>::from([]);
        for event in event_collector.poll_events() {
            let event_messages = view.on_event(event);
            unprocessed_messages.extend(event_messages);
        }

        // Update widgets
        let mut tasks: Vec<app::Task> = vec![];
        while let Some(msg) = unprocessed_messages.pop_front() {
            tasks.extend(app.update(msg));
        }

        // Dispatch tasks
        for t in tasks.iter() {
            match t {
                app::Task::Quit => break 'app,
                app::Task::PlayVoice(_v) => {}
            }
        }
    }
}
