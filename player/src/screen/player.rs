use iced::{
    Element,
    Length::Fill,
    widget::{Space, button, canvas, column, row, scrollable, space},
};
use iced_renderer::geometry::path::lyon_path::geom::euclid::Length;

use crate::widget::{PianoRollRenderer, TrackRenderer};

pub struct Player {
    midi_file: Option<midi::File>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile(String),
}

pub enum Action {
    None,
}

impl Player {
    pub fn new() -> Self {
        Player { midi_file: None }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::OpenFile(filepath) => {
                self.midi_file = Some(midi::File::new(filepath));
            }
        }
        Action::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(ref midi_file) = self.midi_file {
            let piano_roll_renderer = PianoRollRenderer::new();
            let track_renderer = TrackRenderer::new(&midi_file.tracks[1]);

            let canvas_header = canvas(piano_roll_renderer).width(50).height(3500);

            let track_canvas =
                scrollable(canvas(track_renderer).width(3000).height(3500)).horizontal();

            scrollable(row![canvas_header, track_canvas]).into()
        } else {
            let open_file =
                button("click to load file").on_press(Message::OpenFile("./test.mid".to_string()));

            let v_fill = Space::default().height(Fill);
            let h_fill = Space::default().width(Fill);

            return column![
                v_fill,
                row![h_fill, open_file, Space::default().width(Fill)],
                Space::default().height(Fill)
            ]
            .into();
        }
    }
}
