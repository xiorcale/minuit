use iced::{Point, Rectangle, Renderer, Theme, mouse::Cursor, widget::canvas};

const NUM_MIDI_NOTES: u8 = 128;
const NOTE_WIDTH: u8 = 10;
const NOTE_HEIGHT: u8 = 20;

pub struct NoteHeaderProgram {}

impl NoteHeaderProgram {
    pub fn new() -> Self {
        NoteHeaderProgram {}
    }
}

impl<Message> canvas::Program<Message> for NoteHeaderProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for key in (0..NUM_MIDI_NOTES).rev() {
            let note_name = midi::Note::name_from_key(key);

            let y = (NUM_MIDI_NOTES - key) as f32 * NOTE_HEIGHT as f32;

            let note_text = canvas::Text {
                content: note_name,
                position: Point::new(NOTE_WIDTH.into(), y),
                align_y: iced::alignment::Vertical::Center,
                ..canvas::Text::default()
            };

            frame.fill_text(note_text);
        }

        vec![frame.into_geometry()]
    }
}
