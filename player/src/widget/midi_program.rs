use iced::{
    Color, Renderer, border,
    widget::canvas::{self, Frame},
};

const NUM_MIDI_NOTES: f32 = 128.0;
const NOTE_WIDTH: f32 = 10.0;
const NOTE_HEIGHT: f32 = 30.0;

pub struct MidiProgram {
    pub midi_file: midi::File,
}

impl MidiProgram {
    pub fn new() -> Self {
        let file = midi::File::new();
        MidiProgram { midi_file: file }
    }

    fn draw_note(&self, note: &midi::Note, frame: &mut Frame<Renderer>) {
        let x = (note.start_time as f32 / NOTE_WIDTH) as f32;
        let y = (NUM_MIDI_NOTES - note.key as f32) * NOTE_HEIGHT - (NOTE_HEIGHT / 2.0);

        let top_left = iced::Point::new(x, y);
        let size = iced::Size::new(note.duration as f32 / NOTE_WIDTH, NOTE_HEIGHT);

        let note_rectangle = canvas::Path::rounded_rectangle(top_left, size, border::radius(2));

        frame.fill(&note_rectangle, Color::BLACK);
    }
}

impl<Message> canvas::Program<Message> for MidiProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced_renderer::core::Theme,
        bounds: iced::Rectangle,
        _cursor: iced_renderer::core::mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        self.midi_file.tracks[1].notes.iter().for_each(|note| {
            self.draw_note(note, &mut frame);
        });

        vec![frame.into_geometry()]
    }
}
