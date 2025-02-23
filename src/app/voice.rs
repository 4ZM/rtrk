// Copyright (C) 2025 Anders Sundman <anders@4zm.org>
//
// This file is part of RTRK - The Rust Tracker
//
// RTRK is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RTRK is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RTRK. If not, see <https://www.gnu.org/licenses/>.

use std::cell::RefCell;
use std::rc::Rc;

use crate::impl_focusable_with_focuschain;
use crate::synth;
use crate::uifw::interaction::Event;

use crate::uifw::pos::Pos;
use crate::uifw::widget::focus::{FocusChain, FocusableRc};
use crate::uifw::widget::textbox;
use crate::uifw::widget::textbox::{textbox_rc, TextBoxRc, TextBoxView};
use crate::uifw::widget::{Focusable, Task, View, Widget};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    NextFocus,
    PrevFocus,
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

    pub fn get_voice(&self) -> Option<synth::Voice> {
        let osc = self.osc_txt.borrow().text().parse::<usize>();
        let osc = match osc {
            Ok(0) => synth::Oscilator::Sine,
            Ok(1) => synth::Oscilator::Saw,
            Ok(2) => synth::Oscilator::Square,
            _ => return None,
        };

        // Populate rest of voice data

        Some(synth::Voice {
            osc,
            env: None,
            lp: None,
            hp: None,
        })
    }
}

impl Widget<Message, (), VoiceView> for Voice {
    fn update(&mut self, msg: Message) -> Vec<Task<()>> {
        match msg {
            Message::NextFocus => self.next_focus(),
            Message::PrevFocus => self.prev_focus(),
            Message::Osc(m) => {
                self.osc_txt.borrow_mut().update(m);
            }
            Message::Env(m) => {
                self.env_txt.borrow_mut().update(m);
            }
            Message::Flt(m) => {
                self.flt_txt.borrow_mut().update(m);
            }
        };
        vec![]
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
    fn draw(&self, renderer: &mut dyn crate::uifw::interaction::Renderer) {
        self.osc_txt.draw(renderer);
        self.env_txt.draw(renderer);
        self.flt_txt.draw(renderer);
    }
    fn on_event(&self, e: Event) -> Vec<Message> {
        match e {
            Event::NextFocus => return vec![Message::NextFocus],
            Event::PrevFocus => return vec![Message::PrevFocus],
            _ => {}
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

pub mod list {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::app::voice::{voice_rc, VoiceRc, VoiceView};
    use crate::cycle::Cycle;
    use crate::synth;
    use crate::uifw::interaction::Event;
    use crate::uifw::pos::Pos;
    use crate::uifw::widget::focus::{FocusChain, FocusableRc};
    use crate::uifw::widget::label::{label, Label};
    use crate::uifw::widget::{Focusable, Task, View, Widget};
    use crate::{app::voice, impl_focusable_with_focuschain};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        PrevFocus,
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

        pub fn get_selected_voice(&self) -> Option<synth::Voice> {
            self.voices[*self.selected_voice_idx].borrow().get_voice()
        }
    }

    impl Widget<Message, (), VoiceListView> for VoiceList {
        fn update(&mut self, msg: Message) -> Vec<Task<()>> {
            match msg {
                Message::NextFocus => self.next_focus(),
                Message::PrevFocus => self.prev_focus(),
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
                Message::Voice(idx, vm) => {
                    self.voices[idx].borrow_mut().update(vm);
                }
            };
            vec![]
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
        fn draw(&self, renderer: &mut dyn crate::uifw::interaction::Renderer) {
            self.idx_labels.iter().for_each(|v| v.draw(renderer));
            self.voices.iter().for_each(|v| v.draw(renderer));
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            match e {
                Event::NextFocus => return vec![Message::NextFocus],
                Event::PrevFocus => return vec![Message::PrevFocus],
                _ if !self.has_focus => return vec![],
                _ => {}
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
