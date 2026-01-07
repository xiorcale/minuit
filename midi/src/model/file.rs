use std::{
    cmp::{max, min},
    io::{Read, Result, Seek},
};

use crate::model::{Event, EventKind, MetaEventKind, Note, Track};

use byteorder::{BigEndian, ReadBytesExt};

pub struct File {
    pub tracks: Vec<Track>,
    pub tempo: u32,
    pub bpm: u32,
}

impl File {
    pub fn new() -> Self {
        let mut file = File {
            tracks: vec![],
            tempo: 0,
            bpm: 0,
        };

        file.parse_file("./test.mid");
        file.build_notes();

        file
    }

    fn parse_file(&mut self, filename: &str) -> bool {
        let mut file = match std::fs::File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("couldn't parse file: {e}");
                return false;
            }
        };

        // Read MIDI header (fixed sized)
        let file_id = file
            .read_u32::<BigEndian>()
            .expect("couldn't read header file id");

        let header_length = file
            .read_u32::<BigEndian>()
            .expect("couldn't read header length");

        let format = file
            .read_u16::<BigEndian>()
            .expect("couldn't read header format");

        let track_chunks = file
            .read_u16::<BigEndian>()
            .expect("couldn't read header track chunks");

        let division = file
            .read_u16::<BigEndian>()
            .expect("couldn't read header division");

        // Read tracks
        for chunk in 0..track_chunks {
            println!("===== New Track");

            // Read track header
            let track_id = file
                .read_u32::<BigEndian>()
                .expect("couldn't read track id");

            println!("track id: {track_id}");

            let track_length = file
                .read_u32::<BigEndian>()
                .expect("couldn't read track length");

            println!("track length: {track_length}");

            let mut end_of_track = false;
            let mut previous_status = 0;

            let mut track = Track::new();

            while !end_of_track {
                // Read Timecode
                let status_time_delta =
                    File::read_value(&mut file).expect("couldn't read status delta time");

                let status_candidate = file.read_u8().expect("couldn't read status");

                let status = if status_candidate < 0x80 {
                    file.seek(std::io::SeekFrom::Current(-1))
                        .expect("couldn't seek previous position");

                    previous_status
                } else {
                    status_candidate
                };

                let maybe_event_kind = EventKind::try_from(status & 0xF0);

                if maybe_event_kind.is_err() {
                    println!("Unknown status bytes: {status}");
                    continue;
                }

                match maybe_event_kind.unwrap() {
                    EventKind::VoiceNoteOff => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let note_id = file.read_u8().expect("couldn't read note id");
                        let note_velocity = file.read_u8().expect("couldn't read note velocity");

                        track.events.push(Event {
                            kind: EventKind::VoiceNoteOff,
                            key: note_id,
                            velocity: note_velocity,
                            delta_tick: status_time_delta,
                        });
                    }
                    EventKind::VoiceNoteOn => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let note_id = file.read_u8().expect("couldn't read note id");
                        let note_velocity = file.read_u8().expect("couldn't read note velocity");

                        let event_kind = if note_velocity == 0 {
                            EventKind::VoiceNoteOff
                        } else {
                            EventKind::VoiceNoteOn
                        };

                        track.events.push(Event {
                            kind: event_kind,
                            key: note_id,
                            velocity: note_velocity,
                            delta_tick: status_time_delta,
                        });
                    }
                    EventKind::VoiceAftertouch => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let note_id = file.read_u8().expect("couldn't read note id");
                        let note_velocity = file.read_u8().expect("couldn't read note velocity");

                        track.events.push(Event {
                            kind: EventKind::VoiceAftertouch,
                            key: note_id,
                            velocity: note_velocity,
                            delta_tick: status_time_delta,
                        });
                    }
                    EventKind::VoiceControlChange => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let note_id = file.read_u8().expect("couldn't read note id");
                        let note_velocity = file.read_u8().expect("couldn't read note velocity");

                        track.events.push(Event {
                            kind: EventKind::VoiceControlChange,
                            key: note_id,
                            velocity: note_velocity,
                            delta_tick: status_time_delta,
                        })
                    }
                    EventKind::VoiceProgramChange => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let program_id = file.read_u8().expect("couldn't read program id");
                    }
                    EventKind::VoiceChannelPressure => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let channel_pressure =
                            file.read_u8().expect("couldn't read channel pressure");
                    }
                    EventKind::VoicePitchBend => {
                        previous_status = status;

                        let channel = status & 0x0F;
                        let ls7b = file.read_u8().expect("couldn't read ls7b");
                        let ms7b = file.read_u8().expect("couldn't read ms7b");
                    }
                    EventKind::SystemExclusive => {
                        previous_status = 0;

                        if status == 0xF0 {
                            let value = File::read_value(&mut file).expect("couldn't read value");
                            let string = File::read_string(&mut file, value as usize)
                                .expect("couldn't read string");

                            println!("System Exclusive Begin: {string}");
                        }

                        if status == 0xF7 {
                            let value = File::read_value(&mut file).expect("couldn't read value");
                            let string = File::read_string(&mut file, value as usize)
                                .expect("couldn't read string");

                            println!("System Exclusive End: {string}");
                        }

                        if status == 0xFF {
                            let meta_event_kind =
                                file.read_u8().expect("could't read meta event kind");

                            let length =
                                File::read_value(&mut file).expect("couldn't read length") as usize;

                            let maybe_meta_event_kind = MetaEventKind::try_from(meta_event_kind);

                            if maybe_meta_event_kind.is_err() {
                                println!("unkown meta event kind: {meta_event_kind}");
                                continue;
                            }

                            match maybe_meta_event_kind.unwrap() {
                                MetaEventKind::Sequence => {
                                    let byte1 = file.read_u8().expect("couldn't read byte 1");
                                    let byte2 = file.read_u8().expect("couldn't read byte 2");
                                    println!("Sequence number {byte1} {byte2}")
                                }
                                MetaEventKind::Text => {
                                    let text = File::read_string(&mut file, length)
                                        .expect("couldn't read text");
                                    println!("Text: {text}");
                                }
                                MetaEventKind::Copyright => {
                                    let copyright = File::read_string(&mut file, length)
                                        .expect("couldn't read copyright");
                                    println!("Copyright: {copyright}");
                                }
                                MetaEventKind::TrackName => {
                                    let track_name = File::read_string(&mut file, length)
                                        .expect("couldn't read track name");
                                    track.set_name(track_name);
                                }
                                MetaEventKind::InstrumentName => {
                                    let instrument_name = File::read_string(&mut file, length)
                                        .expect("couldn't read instrument name");
                                    track.set_instrument(instrument_name);
                                }
                                MetaEventKind::Lyrics => {
                                    let lyrics = File::read_string(&mut file, length)
                                        .expect("couldn't read lyrics");
                                    println!("Lyrics: {lyrics}");
                                }
                                MetaEventKind::Marker => {
                                    let marker = File::read_string(&mut file, length)
                                        .expect("couldn't read marker");
                                    println!("Marker: {marker}");
                                }
                                MetaEventKind::CuePoint => {
                                    let cue_point = File::read_string(&mut file, length)
                                        .expect("couldn't read cue point");
                                    println!("Cue Point: {cue_point}");
                                }
                                MetaEventKind::ChannelPrefix => {
                                    let prefix = file.read_u8().expect("couldn't read prefix");
                                    println!("Prefix: {prefix}");
                                }
                                MetaEventKind::EndOfTrack => {
                                    end_of_track = true;
                                }
                                MetaEventKind::SetTempo => {
                                    if self.tempo == 0 {
                                        self.tempo = file
                                            .read_u24::<BigEndian>()
                                            .expect("couldn't read tempo");

                                        self.bpm = if self.tempo != 0 {
                                            60_000_000u32 / self.tempo
                                        } else {
                                            0
                                        };
                                    }
                                }
                                MetaEventKind::SMPTEOffset => {
                                    let h = file.read_u8().expect("couldn't read H");
                                    let m = file.read_u8().expect("couldn't read M");
                                    let s = file.read_u8().expect("couldn't read S");
                                    let fr = file.read_u8().expect("couldn't read FR");
                                    let ff = file.read_u8().expect("couldn't read FF");

                                    println!("SMPTEOffset: H:{h} M:{m} S:{s} FR:{fr} FF:{ff}");
                                }
                                MetaEventKind::TimeSignature => {
                                    let time_signature_numerator = file
                                        .read_u8()
                                        .expect("couldn't read time signature numerator");

                                    let time_signature_denominator = 2u32
                                        << file
                                            .read_u8()
                                            .expect("couldn't read time signature denominator");

                                    let clocks_per_tick =
                                        file.read_u8().expect("couldn't read clocks per tick");

                                    let thirty_two_per_24_clocks =
                                        file.read_u8().expect("couldn't read 32 per 24 clocks");

                                    println!(
                                        "Time Signature: {time_signature_numerator}/{time_signature_denominator}"
                                    );
                                    println!("ClocksPerTick: {clocks_per_tick}");
                                    println!("32per24Clocks: {thirty_two_per_24_clocks}");
                                }
                                MetaEventKind::KeySignature => {
                                    let key_signature =
                                        file.read_u8().expect("couldn't read key signature");
                                    let minor_key =
                                        file.read_u8().expect("couldn't read minor key");

                                    println!("Key Signature: {key_signature}");
                                    println!("Minor Key: {minor_key}");
                                }
                                MetaEventKind::SequenceSpecific => {
                                    let sequence_specific = File::read_string(&mut file, length)
                                        .expect("couldn't read sequence specific");

                                    println!("Sequence Specific: {sequence_specific}");
                                }
                            }
                        }
                    }
                }
            }

            self.tracks.push(track);
        }

        true
    }

    fn build_notes(&mut self) {
        for track in self.tracks.as_mut_slice() {
            let mut current_time = 0u32;

            let mut notes_being_processed: Vec<Note> = vec![];

            for event in track.events.as_slice() {
                current_time += event.delta_tick;

                if event.kind == EventKind::VoiceNoteOn {
                    notes_being_processed.push(Note {
                        key: event.key,
                        velocity: event.velocity,
                        start_time: current_time,
                        duration: 0,
                    });
                }

                if event.kind == EventKind::VoiceNoteOff {
                    if let Some(index) = notes_being_processed
                        .iter()
                        .position(|n| n.key == event.key)
                    {
                        let mut note = notes_being_processed.remove(index);
                        note.duration = current_time - note.start_time;

                        track.min_note = min(track.min_note, note.key);
                        track.max_note = max(track.max_note, note.key);
                        track.notes.push(note);
                    }
                }
            }
        }
    }

    fn read_string<R: Read>(reader: &mut R, length: usize) -> Result<String> {
        let mut buf = vec![0u8; length];

        reader.read_exact(&mut buf)?;

        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    /// Reads a MIDI variable-length quantity (up to 32 bits) from the reader.
    /// Returns the decoded value (u32). Errors on EOF or IO failures.
    ///
    /// Algorithm:
    /// - Read bytes one at a time.
    /// - Use the low 7 bits of each byte.
    /// - If MSB is 1, more bytes follow; continue until a byte with MSB==0.
    /// - Assemble value by shifting previous bits left by 7 and OR-ing the next 7 bits.
    fn read_value<R: Read>(reader: &mut R) -> Result<u32> {
        let mut value: u32;

        // Read first byte
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let mut byte = buf[0];

        if (byte & 0x80) == 0 {
            // Single-byte value
            return Ok(u32::from(byte));
        }

        // Multi-byte value: use low 7 bits of first byte
        value = u32::from(byte & 0x7F);

        // Read continuation bytes
        loop {
            reader.read_exact(&mut buf)?;
            byte = buf[0];
            value = (value << 7) | u32::from(byte & 0x7F);
            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_midi() {
        let file = File::new();
        let bpm = file.bpm;
        println!("bpm: {bpm}");

        println!("Tracks: {}", file.tracks.len());

        for track in file.tracks {
            println!();
            println!("track name: {}", track.name);
            for event in track.events {
                println!("event: {:?}", event);
            }
        }
    }
}
