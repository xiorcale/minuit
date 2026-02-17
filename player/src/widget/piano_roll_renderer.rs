use iced::{Point, Rectangle, Renderer, Size, Theme, mouse::Cursor, widget::canvas};
use iced_renderer::geometry::{Path, Stroke};

use crate::widget::canvas_config::{NOTE_HEIGHT, NOTE_WIDTH};

pub struct PianoRollRenderer<'a> {
    track: &'a midi::Track,
}

impl<'a> PianoRollRenderer<'a> {
    pub fn new(track: &'a midi::Track) -> Self {
        PianoRollRenderer { track }
    }
}

impl<'a, Message> canvas::Program<Message> for PianoRollRenderer<'a> {
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

        for (i, key) in (self.track.min_note..=self.track.max_note)
            .rev()
            .enumerate()
        {
            let note_name = midi::Note::name_from_key(key);

            let y = i as f32 * NOTE_HEIGHT;

            let note_text = canvas::Text {
                content: note_name,
                position: Point::new(NOTE_WIDTH, y + NOTE_HEIGHT / 2.0),
                align_y: iced::alignment::Vertical::Center,
                ..canvas::Text::default()
            };

            frame.fill_text(note_text);

            frame.stroke_rectangle(
                Point::new(0.0, y),
                Size::new(bounds.size().width, NOTE_HEIGHT),
                Stroke::default().with_width(0.25),
            );
        }

        // fix outter line not showing up at 0.25 width
        let vertical_left_line = Path::line(
            Point::new(0.0, 0.0),
            Point::new(0.0, self.track.note_range() as f32 * NOTE_HEIGHT),
        );

        let horizontal_top_line =
            Path::line(Point::new(0.0, 0.0), Point::new(bounds.size().width, 0.0));

        frame.stroke(&vertical_left_line, Stroke::default().with_width(0.5));
        frame.stroke(&horizontal_top_line, Stroke::default().with_width(0.5));

        vec![frame.into_geometry()]
    }
}
