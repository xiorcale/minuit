use crate::model::Event;
use crate::model::Note;

pub struct Track {
    pub name: String,
    pub instrument: String,
    pub events: Vec<Event>,
    pub notes: Vec<Note>,
    pub max_note: u8,
    pub min_note: u8,
}

impl Track {
    pub fn new() -> Self {
        Track {
            name: "".to_string(),
            instrument: "".to_string(),
            events: vec![],
            notes: vec![],
            max_note: 64,
            min_note: 64,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_instrument(&mut self, instrument: String) {
        self.instrument = instrument;
    }
}
