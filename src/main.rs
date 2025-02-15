mod ui;

fn main() {
    let ui = ui::UI::new();
    ui.start().expect("");
}
