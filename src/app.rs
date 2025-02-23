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

mod voice;

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

use crate::synth::{AsyncSynth, Note};
use crate::uifw::interaction::Event;
use crate::uifw::pos::Pos;
use crate::uifw::widget::button::{button_rc, ButtonRc, ButtonView};
use crate::uifw::widget::focus::{FocusChain, FocusableRc};
use crate::uifw::widget::label::{label, Label};
use crate::uifw::widget::{Focusable, Task, View, Widget};
use crate::uifw::TaskProcessor;
use crate::{impl_focusable_with_focuschain, synth};
use synth::rodio::RodioAudioSink;
use voice::list::{voicelist_rc, VoiceListRc, VoiceListView};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AppTask {
    PlayVoice(synth::Voice),
    StopVoice,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Quit,
    Play,
    Stop,
    Rewind,
    NextFocus,
    PrevFocus,
    VoiceList(voice::list::Message),
}

pub struct App {
    voices: VoiceListRc,
    play_btn: ButtonRc<Message>,
    stop_btn: ButtonRc<Message>,
    rewind_btn: ButtonRc<Message>,
    focus_chain: FocusChain,
}

pub struct AppTaskProcessor {
    synth: AsyncSynth,
}
impl AppTaskProcessor {
    pub fn new() -> Self {
        Self {
            synth: AsyncSynth::new(|| RodioAudioSink::new(4), 4),
        }
    }
}
impl TaskProcessor<AppTask> for AppTaskProcessor {
    fn process(&mut self, task: &AppTask) {
        let channel = 0;
        match task {
            AppTask::PlayVoice(v) => self
                .synth
                .send(synth::Message::Play(*v, channel, Note::A))
                .expect(""),
            AppTask::StopVoice => self.synth.send(synth::Message::Stop(channel)).expect(""),
        }
    }
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

impl Widget<Message, AppTask, AppView> for App {
    fn update(&mut self, msg: Message) -> Vec<Task<AppTask>> {
        match msg {
            Message::Quit => return vec![Task::Quit],
            Message::VoiceList(m) => {
                return self.voices.borrow_mut().update(m);
            }
            Message::Rewind => {}
            Message::Stop => {}
            Message::Play => {}
            Message::NextFocus => self.next_focus(),
            Message::PrevFocus => self.prev_focus(),
        };
        vec![]
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
    fn draw(&self, renderer: &mut dyn crate::uifw::interaction::Renderer) {
        self.skin.draw(renderer);
        self.voices.draw(renderer);
        self.rewind_btn.draw(renderer);
        self.stop_btn.draw(renderer);
        self.play_btn.draw(renderer);
    }

    fn on_event(&self, e: Event) -> Vec<Message> {
        match e {
            Event::Quit => return vec![Message::Quit],
            Event::NextFocus => return vec![Message::NextFocus],
            Event::PrevFocus => return vec![Message::PrevFocus],
            _ => {}
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
