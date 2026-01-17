pub struct Note {
    pub key: u8,
    pub velocity: u8,
    pub start_time: u32,
    pub duration: u32,
}

fn pitch_to_string(value: u8) -> String {
    match value {
        0 => "C".to_string(),
        1 => "C#".to_string(),
        2 => "D".to_string(),
        3 => "D#".to_string(),
        4 => "E".to_string(),
        5 => "F".to_string(),
        6 => "F#".to_string(),
        7 => "G".to_string(),
        8 => "G#".to_string(),
        9 => "A".to_string(),
        10 => "A#".to_string(),
        11 => "B".to_string(),
        _ => panic!("unkown LetterNote"),
    }
}

impl Note {
    /// Returns a `String` representation of the pitch and octave of the given midi key (C3, A2, ...).
    pub fn name_from_key(key: u8) -> String {
        let pitch = key % 12;
        let octave = (key / 12) as i8 - 2;

        format!("{}{}", pitch_to_string(pitch), octave)
    }
}
