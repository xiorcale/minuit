use iced::{
    Color, Point, Renderer, border,
    widget::canvas::{self, Frame},
};

pub struct MidiProgram {
    pub midi_file: midi::File,
}

impl MidiProgram {
    pub fn new() -> Self {
        let file = midi::File::new();
        MidiProgram { midi_file: file }
    }

    fn draw_note(&self, note: &midi::Note, frame: &mut Frame<Renderer>) {
        let top_left = iced::Point::new(
            (note.start_time / 10 + 100) as f32,
            2560.0 - (note.key as f32) * 20.0 - 5.0,
        );
        let size = iced::Size::new((note.duration / 10) as f32, 10.0);

        let note_rectangle = canvas::Path::rounded_rectangle(top_left, size, border::radius(2));

        frame.fill(&note_rectangle, Color::BLACK);
    }

    fn draw_header(&self, frame: &mut Frame<Renderer>) {
        for key in (0..128).rev() {
            let note_name = midi::Note::name_from_key(key);

            let note_text = canvas::Text {
                content: note_name,
                position: Point::new(10.0, 2560.0 - (key as f32 * 20.0)),
                align_y: iced::alignment::Vertical::Center,
                ..canvas::Text::default()
            };

            frame.fill_text(note_text);
        }
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

        //self.draw_header(&mut frame);

        self.midi_file.tracks[1].notes.iter().for_each(|note| {
            self.draw_note(note, &mut frame);
        });

        vec![frame.into_geometry()]
    }
}
