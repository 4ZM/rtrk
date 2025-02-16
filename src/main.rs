mod cycle;
mod interaction;
mod pos;
mod runtime;
mod term;
mod widget;

mod app {

    const SKIN: &str = r#"
┏━━━━━━━━━[ rtrk ]━━━━━━━━━━━━━━━━━━━━━━━━━━ , ━━━━━━ [v0.1] ━━━ =^..^= ━━━━━━━┓
┃                                     ______/ \_ _/\______,___/\ ___' _____,   ┃
┃     .                               \         \   ____/       \   :/    /    ┃
┃     :                               /    <    /:  \ \    >    /   ;   _/     ┃
┃     :                              /         < |   \/       <<         \     ┃
┃     :                             /      :    \|    \    ;    \   ,     \    ┃
┃     :                             \      |     \    /    |     \  :      \   ┃
┃     '                              \  ___^_____/   /\____|     /__:       \  ┃
┃                                     \/   ;      \ /  4ZM  \___/   |_______/  ┃
┠──────────────────────────────────────────────────'───────────────────────────┨
┃ ▚▚▚▚▚▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚█████                            |    |   |   |        ┃
┠──────────────────────────────────────────────────────────────────────────────┨
┃ ## : ▁▂▃▄▅▆▇██▆▅▃  :  ▆▅▃▁▂▃▄▅▆▇█   :  ▅▆▇█▅▆▇█▆▅▃▁  : ▃▁▁▁▂▃▄▅▆▇█▆▃  :  gFx ┃
┠──────────────────────────────────────────────────────────────────────────────┨
┃ 09 . C#4 1 A0 101  .  --- - -- ---  .  --- - -- ---  .  --- - -- ---  .  2FF ┃
┃ 0A : --- - -- ---  :  C#4 1 A0 101  :  --- - -- ---  :  --- - -- ---  :  --- ┃
┃ 0B > --- - FF --- <:> --- - -- --- <:> --- - -- --- <:> --- - -- --- <:> --- ┃
┃ 0C : --- - -- 105  :  --- - -- ---  :  --- - -- ---  :  --- - -- ---  :  000 ┃
┃ 0D ' A-5 4 20 ---  '  --- - -- ---  '  C#4 1 A0 101  '  --- - -- ---  '  --- ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#;

    use crate::impl_focusable_with_focuschain;
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::voice_list;
    use crate::voice_list::{voicelist_rc, VoiceListRc, VoiceListView};
    use crate::widget::button::{button_rc, ButtonRc, ButtonView};
    use crate::widget::focus::{FocusChain, FocusableRc};
    use crate::widget::label::{label, Label};
    use crate::widget::{Focusable, View, Widget};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        Play,
        Stop,
        Rewind,
        NextFocus,
        VoiceList(voice_list::Message),
    }

    pub struct App {
        voices: VoiceListRc,
        play_btn: ButtonRc<Message>,
        stop_btn: ButtonRc<Message>,
        rewind_btn: ButtonRc<Message>,
        focus_chain: FocusChain,
    }

    impl App {
        pub fn new() -> Self {
            let voices = voicelist_rc();
            let play_btn = button_rc(">", Message::Play);
            let stop_btn = button_rc(".", Message::Stop);
            let rewind_btn = button_rc("<<", Message::Rewind);

            let mut focus_chain = FocusChain::new();
            focus_chain.push(voices.clone() as FocusableRc);
            focus_chain.push(rewind_btn.clone() as FocusableRc);
            focus_chain.push(stop_btn.clone() as FocusableRc);
            focus_chain.push(play_btn.clone() as FocusableRc);

            Self {
                voices,
                rewind_btn,
                stop_btn,
                play_btn,
                focus_chain,
            }
        }
    }

    impl Widget<Message, AppView> for App {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::VoiceList(m) => self.voices.borrow_mut().update(m),
                Message::Rewind => {}
                Message::Stop => {}
                Message::Play => {}
                Message::NextFocus => self.next_focus(),
            };
        }

        fn view(&self, pos: Pos) -> AppView {
            AppView {
                voices: self.voices.borrow().view(pos + Pos { r: 3, c: 3 }),
                skin: label(Pos { r: 0, c: 0 }, SKIN),
                rewind_btn: self.rewind_btn.borrow().view(pos + Pos { r: 11, c: 58 }),
                stop_btn: self.stop_btn.borrow().view(pos + Pos { r: 11, c: 63 }),
                play_btn: self.play_btn.borrow().view(pos + Pos { r: 11, c: 67 }),
            }
        }
    }
    impl_focusable_with_focuschain!(App, focus_chain);

    pub struct AppView {
        voices: VoiceListView,
        rewind_btn: ButtonView<Message>,
        stop_btn: ButtonView<Message>,
        play_btn: ButtonView<Message>,
        skin: Label,
    }
    impl View<Message> for AppView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            self.skin.draw(renderer);
            self.voices.draw(renderer);
            self.rewind_btn.draw(renderer);
            self.stop_btn.draw(renderer);
            self.play_btn.draw(renderer);
        }

        fn on_event(&self, e: Event) -> Vec<Message> {
            if let Event::NextFocus = e {
                return vec![Message::NextFocus];
            }

            let mut msgs: Vec<Message> = vec![];
            self.voices
                .on_event(e)
                .iter()
                .for_each(|&m| msgs.push(Message::VoiceList(m)));
            self.rewind_btn
                .on_event(e)
                .iter()
                .for_each(|&m| msgs.push(m));
            self.stop_btn.on_event(e).iter().for_each(|&m| msgs.push(m));
            self.play_btn.on_event(e).iter().for_each(|&m| msgs.push(m));
            msgs
        }
    }
}

mod voice {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::impl_focusable_with_focuschain;
    use crate::interaction::Event;

    use crate::pos::Pos;
    use crate::widget::focus::{FocusChain, FocusableRc};
    use crate::widget::textbox;
    use crate::widget::textbox::{textbox_rc, TextBoxRc, TextBoxView};
    use crate::widget::{Focusable, View, Widget};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Osc(textbox::Message),
        Env(textbox::Message),
        Flt(textbox::Message),
    }

    pub struct Voice {
        focus_chain: FocusChain,
        osc_txt: TextBoxRc,
        env_txt: TextBoxRc,
        flt_txt: TextBoxRc,
    }

    impl Voice {
        pub fn new() -> Self {
            let osc_txt = textbox_rc(1);
            let env_txt = textbox_rc(8);
            let flt_txt = textbox_rc(6);

            let mut focus_chain = FocusChain::new();
            focus_chain.push(osc_txt.clone() as FocusableRc);
            focus_chain.push(env_txt.clone() as FocusableRc);
            focus_chain.push(flt_txt.clone() as FocusableRc);

            Self {
                focus_chain,
                osc_txt,
                env_txt,
                flt_txt,
            }
        }
    }

    impl Widget<Message, VoiceView> for Voice {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::NextFocus => self.next_focus(),
                Message::Osc(m) => self.osc_txt.borrow_mut().update(m),
                Message::Env(m) => self.env_txt.borrow_mut().update(m),
                Message::Flt(m) => self.flt_txt.borrow_mut().update(m),
            };
        }

        fn view(&self, pos: Pos) -> VoiceView {
            VoiceView {
                osc_txt: self.osc_txt.borrow().view(pos + Pos { r: 0, c: 0 }),
                env_txt: self.env_txt.borrow().view(pos + Pos { r: 0, c: 2 }),
                flt_txt: self.flt_txt.borrow().view(pos + Pos { r: 0, c: 11 }),
            }
        }
    }
    impl_focusable_with_focuschain!(Voice, focus_chain);

    pub struct VoiceView {
        osc_txt: TextBoxView,
        env_txt: TextBoxView,
        flt_txt: TextBoxView,
    }
    impl View<Message> for VoiceView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            self.osc_txt.draw(renderer);
            self.env_txt.draw(renderer);
            self.flt_txt.draw(renderer);
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            if let Event::NextFocus = e {
                return vec![Message::NextFocus];
            }
            let mut msgs: Vec<Message> = vec![];
            self.osc_txt
                .on_event(e)
                .iter()
                .for_each(|&m| msgs.push(Message::Osc(m)));
            self.env_txt
                .on_event(e)
                .iter()
                .for_each(|&m| msgs.push(Message::Env(m)));
            self.flt_txt
                .on_event(e)
                .iter()
                .for_each(|&m| msgs.push(Message::Flt(m)));

            msgs
        }
    }

    pub type VoiceRc = Rc<RefCell<Voice>>;
    pub fn voice_rc() -> VoiceRc {
        Rc::new(RefCell::new(Voice::new()))
    }
}

mod voice_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::cycle::Cycle;
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::voice::voice_rc;
    use crate::voice::{VoiceRc, VoiceView};
    use crate::widget::focus::{FocusChain, FocusableRc};
    use crate::widget::label::{label, Label};
    use crate::widget::{Focusable, View, Widget};
    use crate::{impl_focusable_with_focuschain, voice};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Up,
        Down,
        Voice(usize, voice::Message),
    }

    pub struct VoiceList {
        focus_chain: FocusChain,
        voices: Vec<VoiceRc>,
        first_voice_idx: Cycle,
        selected_voice_idx: Cycle,
        list_window_len: usize,
    }

    impl VoiceList {
        pub fn new() -> Self {
            let list_window_len = 6;
            let voices: Vec<_> = (0..0x100).map(|_| voice_rc()).collect();

            let mut focus_chain = FocusChain::new();
            focus_chain.push(voices[0].clone() as FocusableRc);

            Self {
                focus_chain,
                voices,
                first_voice_idx: Cycle::new(0, 0x100),
                selected_voice_idx: Cycle::new(0, 0x100),
                list_window_len,
            }
        }

        fn set_voice_focus(&mut self, idx: usize) {
            self.focus_chain.clear();
            self.focus_chain
                .push(self.voices[idx].clone() as FocusableRc);
            self.focus_chain.next_focus();
        }
    }

    impl Widget<Message, VoiceListView> for VoiceList {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::NextFocus => self.next_focus(),
                Message::Up => {
                    if *self.selected_voice_idx == *self.first_voice_idx {
                        self.first_voice_idx -= 1;
                    }
                    self.selected_voice_idx -= 1;
                    self.set_voice_focus(*self.selected_voice_idx);
                }
                Message::Down => {
                    let last_voice_idx = *(self.first_voice_idx + self.list_window_len - 1);
                    if *self.selected_voice_idx == last_voice_idx {
                        self.first_voice_idx += 1;
                    }

                    self.selected_voice_idx += 1;
                    self.set_voice_focus(*self.selected_voice_idx);
                }
                Message::Voice(idx, vm) => self.voices[idx].borrow_mut().update(vm),
            };
        }

        fn view(&self, pos: Pos) -> VoiceListView {
            VoiceListView::new(
                pos,
                &self.voices,
                self.first_voice_idx,
                self.list_window_len,
                self.focus_chain.has_focus(),
            )
        }
    }
    impl_focusable_with_focuschain!(VoiceList, focus_chain);

    pub struct VoiceListView {
        voices: Vec<VoiceView>,
        idx_offset: Cycle,
        idx_labels: Vec<Label>,
        has_focus: bool,
    }

    impl VoiceListView {
        pub fn new(
            pos: Pos,
            voices: &Vec<VoiceRc>,
            first_voice_idx: Cycle,
            list_len: usize,
            has_focus: bool,
        ) -> Self {
            let voices: Vec<_> = voices
                .iter()
                .cycle()
                .skip(*first_voice_idx)
                .take(list_len)
                .collect();
            Self {
                voices: voices
                    .iter()
                    .enumerate()
                    .map(|(i, v)| v.borrow().view(pos + Pos { r: i as u16, c: 5 }))
                    .collect(),
                idx_offset: first_voice_idx,
                idx_labels: (0..list_len)
                    .map(|i| {
                        label(
                            pos + Pos { r: i as u16, c: 0 },
                            &format!("{:02X}", *(first_voice_idx + i)),
                        )
                    })
                    .collect(),
                has_focus,
            }
        }
    }
    impl View<Message> for VoiceListView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            self.idx_labels.iter().for_each(|v| v.draw(renderer));
            self.voices.iter().for_each(|v| v.draw(renderer));
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            if let Event::NextFocus = e {
                return vec![Message::NextFocus];
            }

            if !self.has_focus {
                return vec![];
            }
            match e {
                Event::Up => return vec![Message::Up],
                Event::Down => return vec![Message::Down],
                _ => {}
            }

            let mut msgs: Vec<Message> = vec![];
            for (i, v) in self.voices.iter().enumerate() {
                v.on_event(e)
                    .iter()
                    .for_each(|&m| msgs.push(Message::Voice(*(self.idx_offset + i), m)));
            }

            msgs
        }
    }

    pub type VoiceListRc = Rc<RefCell<VoiceList>>;
    pub fn voicelist_rc() -> VoiceListRc {
        Rc::new(RefCell::new(VoiceList::new()))
    }
}

fn main() {
    let mut app = app::App::new();
    runtime::start(&mut app);
}

#[cfg(test)]
mod tests {
    use super::interaction::tests::TestRenderer;
    use super::pos::*;
    use super::widget::*;
    use super::*;
}
