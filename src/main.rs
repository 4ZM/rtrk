mod app;
mod cycle;
mod synth;
mod uifw;

fn main() {
    let mut app = app::App::new();
    uifw::start(&mut app);
}
