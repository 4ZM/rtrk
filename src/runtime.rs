use crate::interaction::{Event, EventCollector, Renderer};
use crate::pos::Pos;
use crate::term;
use crate::widget::{View, Widget};
use std::collections::VecDeque;
use std::time::Duration;

// Runtime
pub fn start<Message, V: View<Message>>(app: &mut dyn Widget<Message, V>) {
    let mut renderer = term::CrosstermRenderer::new(std::io::stdout());
    let event_collector = term::CrosstermEventCollector {};

    'app: loop {
        // Render state
        let view = app.view(Pos { r: 0, c: 0 });
        renderer.clear();
        view.draw(&mut renderer);
        renderer.flush();

        std::thread::sleep(Duration::from_millis(20));

        // Get UI event interactions
        let mut unprocessed_messages = VecDeque::<Message>::from([]);
        for event in event_collector.poll_events() {
            let event_messages = match event {
                Event::Quit => break 'app,
                _ => view.on_event(event),
            };
            unprocessed_messages.extend(event_messages);
        }

        // Update widgets
        while let Some(msg) = unprocessed_messages.pop_front() {
            app.update(msg);
        }
    }
}
