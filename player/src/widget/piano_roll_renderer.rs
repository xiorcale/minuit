use iced::{Point, Rectangle, Renderer, Size, Theme, mouse::Cursor, widget::canvas};
use iced_renderer::geometry::Stroke;

const NUM_MIDI_NOTES: u8 = 127;
const NOTE_WIDTH: u8 = 10;
const NOTE_HEIGHT: u8 = 25;

pub struct PianoRollRenderer {}

impl PianoRollRenderer {
    pub fn new() -> Self {
        PianoRollRenderer {}
    }
}

impl<Message> canvas::Program<Message> for PianoRollRenderer {
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

        for key in (0..=NUM_MIDI_NOTES).rev() {
            let note_name = midi::Note::name_from_key(key);

            let y = (NUM_MIDI_NOTES - key) as f32 * NOTE_HEIGHT as f32;

            let note_text = canvas::Text {
                content: note_name,
                position: Point::new(NOTE_WIDTH.into(), y + NOTE_HEIGHT as f32 / 2.0),
                align_y: iced::alignment::Vertical::Center,
                ..canvas::Text::default()
            };

            frame.fill_text(note_text);

            frame.stroke_rectangle(
                Point::new(0.0, y),
                Size::new(bounds.size().width, NOTE_HEIGHT.into()),
                Stroke::default().with_width(0.25),
            );
        }

        vec![frame.into_geometry()]
    }
}
