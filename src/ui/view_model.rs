use itertools::Itertools;

use crate::ui::synth::view_model::SynthViewModel;
use crate::ui::tracker::view_model::TrackerViewModel;

// Sound is really part of the DataModel
#[derive(Copy, Clone)]
pub struct Sound {
    pub wave_id: Option<u8>,
    pub attack: Option<u8>,
    pub decay: Option<u8>,
    pub sustain: Option<u8>,
    pub release: Option<u8>,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            wave_id: None,
            attack: None,
            decay: None,
            sustain: None,
            release: None,
        }
    }
}

pub struct RootViewModel {
    pub synth: SynthViewModel,
    pub tracks: TrackerViewModel,

    pub version: String,
}

impl RootViewModel {
    pub fn new() -> Self {
        RootViewModel {
            version: Self::version(),
            synth: SynthViewModel::new(),
            tracks: TrackerViewModel::new(),
        }
    }

    fn version() -> String {
        env!("CARGO_PKG_VERSION")
            .split('.')
            .take(2)
            .join(".")
            .to_string()
    }
}
