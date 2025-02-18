#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Oscilator {
    Sine,
    Square,
    Saw,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Envelope {
    pub attack_lvl: f32,
    pub attack_ms: f32,
    pub decay_lvl: f32,
    pub decay_ms: f32,
    pub sustain_lvl: f32,
    pub sustain_min_ms: f32,
    pub release_lvl: f32,
    pub release_ms: f32,
}
impl Envelope {
    pub fn new() -> Self {
        Self {
            attack_lvl: 1.0,
            attack_ms: 0.1,
            decay_lvl: 0.9,
            decay_ms: 0.1,
            sustain_lvl: 0.9,
            sustain_min_ms: 0.5,
            release_lvl: 0.0,
            release_ms: 0.1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Filter {
    pub cutoff: f32,
    pub gain: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Voice {
    pub osc: Oscilator,
    pub env: Envelope,
    pub lp: Option<Filter>,
    pub hp: Option<Filter>,
}
