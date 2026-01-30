use iced::{
    Color, Renderer, border,
    widget::canvas::{self, Frame},
};
use iced_renderer::geometry::Stroke;

use crate::widget::canvas_config::{NOTE_HEIGHT, NOTE_WIDTH, NUM_MIDI_NOTES};

pub struct TrackRenderer<'a> {
    track: &'a midi::Track,
}

impl<'a> TrackRenderer<'a> {
    pub fn new(track: &'a midi::Track) -> Self {
        TrackRenderer { track: track }
    }

    fn draw_note(&self, note: &midi::Note, frame: &mut Frame<Renderer>) {
        let x = (note.start_time as f32 / NOTE_WIDTH) as f32;
        let y = (NUM_MIDI_NOTES - note.key as f32) * NOTE_HEIGHT;

        let top_left = iced::Point::new(x, y);
        let size = iced::Size::new(note.duration as f32 / NOTE_WIDTH, NOTE_HEIGHT);

        let note_rectangle = canvas::Path::rounded_rectangle(top_left, size, border::radius(2));

        frame.fill(&note_rectangle, Color::BLACK);
    }

    fn draw_lines(&self, frame: &mut Frame<Renderer>) {
        for key in (-1..=NUM_MIDI_NOTES as i32).rev() {
            let y = (NUM_MIDI_NOTES - key as f32) as f32 * NOTE_HEIGHT;

            let start_point = iced::Point::new(0.0, y);
            let end_point = iced::Point::new(frame.width(), y);

            let line = canvas::Path::line(start_point, end_point);

            let stroke = Stroke::default().with_width(0.25);

            frame.stroke(&line, stroke);
        }
    }
}

impl<'a, Message> canvas::Program<Message> for TrackRenderer<'a> {
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

        self.draw_lines(&mut frame);

        self.track.notes.iter().for_each(|note| {
            self.draw_note(note, &mut frame);
        });

        vec![frame.into_geometry()]
    }
}
