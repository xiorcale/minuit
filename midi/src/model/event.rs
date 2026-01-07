#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind {
    VoiceNoteOff = 0x80,
    VoiceNoteOn = 0x90,
    VoiceAftertouch = 0xA0,
    VoiceControlChange = 0xB0,
    VoiceProgramChange = 0xC0,
    VoiceChannelPressure = 0xD0,
    VoicePitchBend = 0xE0,
    SystemExclusive = 0xF0,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetaEventKind {
    Sequence = 0x00,
    Text = 0x01,
    Copyright = 0x02,
    TrackName = 0x03,
    InstrumentName = 0x04,
    Lyrics = 0x05,
    Marker = 0x06,
    CuePoint = 0x07,
    ChannelPrefix = 0x20,
    EndOfTrack = 0x2F,
    SetTempo = 0x51,
    SMPTEOffset = 0x54,
    TimeSignature = 0x58,
    KeySignature = 0x59,
    SequenceSpecific = 0x7F,
}

impl TryFrom<u8> for EventKind {
    type Error = ();

    fn try_from(b: u8) -> Result<EventKind, ()> {
        match b {
            0x80 => Ok(EventKind::VoiceNoteOff),
            0x90 => Ok(EventKind::VoiceNoteOn),
            0xA0 => Ok(EventKind::VoiceAftertouch),
            0xB0 => Ok(EventKind::VoiceControlChange),
            0xC0 => Ok(EventKind::VoiceProgramChange),
            0xD0 => Ok(EventKind::VoiceChannelPressure),
            0xE0 => Ok(EventKind::VoicePitchBend),
            0xF0 => Ok(EventKind::SystemExclusive),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for MetaEventKind {
    type Error = ();

    fn try_from(b: u8) -> Result<MetaEventKind, ()> {
        match b {
            0x00 => Ok(MetaEventKind::Sequence),
            0x01 => Ok(MetaEventKind::Text),
            0x02 => Ok(MetaEventKind::Copyright),
            0x03 => Ok(MetaEventKind::TrackName),
            0x04 => Ok(MetaEventKind::InstrumentName),
            0x05 => Ok(MetaEventKind::Lyrics),
            0x06 => Ok(MetaEventKind::Marker),
            0x07 => Ok(MetaEventKind::CuePoint),
            0x20 => Ok(MetaEventKind::ChannelPrefix),
            0x2F => Ok(MetaEventKind::EndOfTrack),
            0x51 => Ok(MetaEventKind::SetTempo),
            0x54 => Ok(MetaEventKind::SMPTEOffset),
            0x58 => Ok(MetaEventKind::TimeSignature),
            0x59 => Ok(MetaEventKind::KeySignature),
            0x7F => Ok(MetaEventKind::SequenceSpecific),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub key: u8,
    pub velocity: u8,
    pub delta_tick: u32,
}
