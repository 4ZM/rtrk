use crate::pos::Pos;
use crate::widget::View;

pub struct Label {
    pos: Pos,
    text: String,
}

impl View<()> for Label {
    fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
        renderer.render_str(self.pos, &format!("{}", &self.text));
    }
}
pub fn label(pos: Pos, text: &str) -> Label {
    Label {
        pos,
        text: text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::{tests::TestRenderer, Event};

    #[test]
    fn label_test() {
        let lbl = label(Pos { r: 0, c: 0 }, "LBL");

        let mut renderer = TestRenderer::new();
        lbl.draw(&mut renderer);
        assert_eq!(renderer.out, "LBL");

        // Can't activate a label
        let msg = lbl.on_event(Event::Activate);
        assert!(msg.is_empty());
    }
}
