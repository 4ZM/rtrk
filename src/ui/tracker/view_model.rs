pub struct TrackerViewModel {
    pub track_list_active: u8,
}

impl TrackerViewModel {
    pub fn new() -> Self {
        TrackerViewModel {
            track_list_active: 42, // TBD Demo value, should start at 0
        }
    }
}
