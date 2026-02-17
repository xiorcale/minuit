use crate::model::Event;
use crate::model::Note;

#[derive(Debug, Clone)]
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
            max_note: 0,
            min_note: 127,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_instrument(&mut self, instrument: String) {
        self.instrument = instrument;
    }

    pub fn note_range(&self) -> u8 {
        // +1 for including both notes in the range
        self.max_note - self.min_note + 1
    }
}
