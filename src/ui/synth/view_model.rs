use crate::ui::view_model::Sound;

pub struct SynthViewModel {
    pub sounds: [Sound; 255],

    pub sounds_list_active: u8,
}

impl SynthViewModel {
    pub fn new() -> Self {
        let mut sounds = [Sound::new(); 255];

        let demo_sound_1 = Sound {
            wave_id: Some(2),
            attack: Some(0x35),
            decay: Some(0xFF),
            sustain: Some(0x10),
            release: Some(0x01),
        };
        let demo_sound_2 = Sound {
            wave_id: Some(4),
            attack: Some(0x10),
            decay: Some(0x10),
            sustain: Some(0xA0),
            release: Some(0x00),
        };

        // DUMMY DATA TBD
        sounds[24] = demo_sound_1;
        sounds[26] = demo_sound_2;

        SynthViewModel {
            sounds,
            sounds_list_active: 23, // TBD Demo value, should start at 0
        }
    }
}
