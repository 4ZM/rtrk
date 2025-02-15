#[derive(Copy, Clone)]
pub enum Action {
    RmChar(usize),
    EnterChar(char),
    Tic,
    NextFocus,
    SelectCurrent,
    Play,
    Stop,
    Quit,
}

pub struct TextBoxModel {
    pub focus: bool,
    pub text: String,
    pub width: usize,
}
impl TextBoxModel {
    pub fn new(text: &str, width: usize, focus: bool) -> Self {
        TextBoxModel {
            focus,
            width,
            text: text.to_string(),
        }
    }
}

pub struct ButtonModel {
    pub focus: bool,
    pub text: String,
    pub on_click: fn(&mut RootModel),
}
impl ButtonModel {
    pub fn new(text: &str, focus: bool, on_click: fn(&mut RootModel)) -> Self {
        ButtonModel {
            focus,
            on_click,
            text: text.to_string(),
        }
    }
}

pub struct RootModel {
    pub playing: bool,
    pub play_button: ButtonModel,
    pub stop_button: ButtonModel,
    pub scroll_text_textbox: TextBoxModel,
}
impl RootModel {
    pub fn update(&mut self, a: Action) {
        match a {
            Action::Play => self.playing = true,
            Action::Stop => self.playing = false,
            Action::NextFocus => self.next_focus(),
            Action::SelectCurrent => {
                if self.play_button.focus {
                    (self.play_button.on_click)(self);
                } else if self.stop_button.focus {
                    (self.stop_button.on_click)(self);
                }
            }
            Action::Tic => {
                if self.playing {
                    self.scroll_text()
                }
            }
            Action::EnterChar(c) => {
                if self.scroll_text_textbox.width > self.scroll_text_textbox.text.len() {
                    self.scroll_text_textbox.text.push(c);
                }
            }
            Action::RmChar(i) => {
                if i < self.scroll_text_textbox.text.len() {
                    self.scroll_text_textbox.text.remove(i);
                }
            }
            _ => {}
        };
    }
    fn scroll_text(&mut self) {
        if let Some(c) = self.scroll_text_textbox.text.pop() {
            self.scroll_text_textbox.text.insert(0, c);
        }
    }

    fn next_focus(&mut self) {
        // Implement some kind of focus chain declarative

        if self.play_button.focus {
            self.play_button.focus = false;
            self.scroll_text_textbox.focus = true;
        } else if self.scroll_text_textbox.focus {
            self.scroll_text_textbox.focus = false;
            self.stop_button.focus = true;
        } else if self.stop_button.focus {
            self.stop_button.focus = false;
            self.play_button.focus = true;
        }
    }

    pub fn new() -> Self {
        RootModel {
            playing: true,
            scroll_text_textbox: TextBoxModel::new(".-''-._", 10, false),
            play_button: ButtonModel::new(">", true, |m: &mut RootModel| m.playing = true),
            stop_button: ButtonModel::new(".", false, |m: &mut RootModel| m.playing = false),
        }
    }
}
