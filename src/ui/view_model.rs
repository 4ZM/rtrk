use itertools::Itertools;

pub struct Pos {
    pub r: u16,
    pub c: u16,
}

#[derive(Copy, Clone)]
pub struct Sound {
    pub wave_id: Option<u8>,
    pub attack: Option<u8>,
    pub decay: Option<u8>,
    pub sustain: Option<u8>,
    pub release: Option<u8>,
}

impl Sound {
    fn new() -> Sound {
        Sound {
            wave_id: None,
            attack: None,
            decay: None,
            sustain: None,
            release: None,
        }
    }
}

pub struct ViewModel {
    pub sounds: [Sound; 255],

    pub sounds_list_active: u8,
    pub track_list_active: u8,

    pub version: String,
}

impl ViewModel {
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
        sounds[2] = demo_sound_1;
        sounds[4] = demo_sound_2;

        ViewModel {
            version: Self::version(),
            sounds,
            sounds_list_active: 23, // TBD Demo value, should start at 0
            track_list_active: 42,  // TBD Demo value, should start at 0
        }
    }

    // pub fn tic(dt_s: f32) {
    //     // Advance UI state
    // }

    fn version() -> String {
        env!("CARGO_PKG_VERSION")
            .split('.')
            .take(2)
            .join(".")
            .to_string()
    }
}
