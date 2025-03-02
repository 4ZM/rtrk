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
┃                                     ______/ \_ _/\_______,___/\ ___' _____,  ┃
┃      .                              \         \    ____/       \   :/    /   ┃
┃      :                              /    <    /:   \ \    >    /   ;   _/    ┃
┃      :                             /         < |    \/       <<         \    ┃
┃      :                            /      :    \|     \    ;    \   ,     \   ┃
┃      :                            \      |     \     /    |     \  :      \  ┃
┃      '                             \  ___^_____/    /\____|     /__:       \ ┃
┃                                     \/   ;      \  /  4ZM  \___/   |_______/ ┃
┠──────────────────────────────────────────────────'/──────────────────────────┨
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

use crate::synth::{AsyncSynth, Frequency};
use crate::uifw::interaction::{CharModifiers, Event};
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
    PlayVoice(synth::Voice, synth::Frequency),
    StopVoice,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Quit,
    Play,
    Stop,
    PlayVoice(synth::Frequency),
    StopVoice,
    Rewind,
    NextFocus,
    PrevFocus,
    NextKbdMode,
    VoiceList(voice::list::Message),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KbdMode {
    Text,
    Claviature,
}
pub struct App {
    voices: VoiceListRc,
    play_btn: ButtonRc<Message>,
    stop_btn: ButtonRc<Message>,
    rewind_btn: ButtonRc<Message>,
    focus_chain: FocusChain,
    kbd_mode: KbdMode,
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
            AppTask::PlayVoice(v, freq) => self
                .synth
                .send(synth::Message::Play(*v, channel, *freq))
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
            kbd_mode: KbdMode::Text,
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
            Message::StopVoice => return vec![Task::App(AppTask::StopVoice)],
            Message::PlayVoice(freq) => {
                if let Some(voice) = self.voices.borrow().get_selected_voice() {
                    return vec![Task::App(AppTask::PlayVoice(voice, freq))];
                }
            }
            Message::NextFocus => self.next_focus(),
            Message::PrevFocus => self.prev_focus(),
            Message::NextKbdMode => {
                self.kbd_mode = match self.kbd_mode {
                    KbdMode::Text => KbdMode::Claviature,
                    KbdMode::Claviature => KbdMode::Text,
                }
            }
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
            kbd_mode: self.kbd_mode,
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
    kbd_mode: KbdMode,
}
impl View<Message> for AppView {
    fn draw(&self, renderer: &mut dyn crate::uifw::interaction::Renderer) {
        self.skin.draw(renderer); // Must draw first since it will overwrite everything
        match self.kbd_mode {
            KbdMode::Text => renderer.render_str(Pos { r: 9, c: 67 }, "#"),
            KbdMode::Claviature => renderer.render_str(Pos { r: 9, c: 67 }, "♫"),
        }
        self.voices.draw(renderer);
        self.rewind_btn.draw(renderer);
        self.stop_btn.draw(renderer);
        self.play_btn.draw(renderer);
    }

    fn on_event(&self, e: Event) -> Vec<Message> {
        match e {
            Event::Quit => return vec![Message::Quit],
            Event::Char('`', _) => return vec![Message::NextKbdMode],
            _ => {}
        }

        // Uppercase all chars
        let mut e = e;
        if let Event::Char(c @ 'a'..='z', m) = e {
            e = Event::Char(c.to_ascii_uppercase(), m);
        }

        fn play_message(Frequency(freq): Frequency, cm: CharModifiers) -> Message {
            Message::PlayVoice(Frequency(
                freq * match cm {
                    CharModifiers::Shift => 2.0,
                    _ => 1.0,
                },
            ))
        }

        if self.kbd_mode == KbdMode::Claviature {
            return match e {
                Event::Char('Z', m) => vec![play_message(synth::Note::C, m)],
                Event::Char('S', m) => vec![play_message(synth::Note::CS, m)],
                Event::Char('X', m) => vec![play_message(synth::Note::D, m)],
                Event::Char('D', m) => vec![play_message(synth::Note::DS, m)],
                Event::Char('C', m) => vec![play_message(synth::Note::E, m)],
                Event::Char('V', m) => vec![play_message(synth::Note::F, m)],
                Event::Char('G', m) => vec![play_message(synth::Note::FS, m)],
                Event::Char('B', m) => vec![play_message(synth::Note::G, m)],
                Event::Char('H', m) => vec![play_message(synth::Note::GS, m)],
                Event::Char('N', m) => vec![play_message(synth::Note::A, m)],
                Event::Char('J', m) => vec![play_message(synth::Note::AS, m)],
                Event::Char('M', m) => vec![play_message(synth::Note::B, m)],
                Event::Char(' ', _) => vec![Message::StopVoice],
                _ => vec![], // Short circuit other input
            };
        }

        match e {
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
