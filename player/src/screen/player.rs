use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{
        Scrollable, Space, button, canvas, column, operation, row,
        scrollable::{Direction, Scrollbar, Viewport},
    },
};

use crate::widget::{PianoRollRenderer, TrackRenderer};

const PIANO_ROLL_ID: &str = "piano_roll";

pub struct Player {
    midi_file: Option<midi::File>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile(String),
    SyncPianoRoll(Viewport),
}

impl Player {
    pub fn new() -> Self {
        Player { midi_file: None }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile(filepath) => {
                self.midi_file = Some(midi::File::new(filepath));
                Task::none()
            }
            Message::SyncPianoRoll(viewport) => {
                operation::scroll_to(PIANO_ROLL_ID, viewport.absolute_offset())
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(ref midi_file) = self.midi_file {
            let piano_roll_renderer = PianoRollRenderer::new();
            let track_renderer = TrackRenderer::new(&midi_file.tracks[1]);

            let canvas_header = Scrollable::new(canvas(piano_roll_renderer).width(50).height(3500))
                .direction(Direction::Vertical(Scrollbar::hidden()))
                .id(PIANO_ROLL_ID);

            let track_canvas = Scrollable::new(canvas(track_renderer).width(3000).height(3500))
                .direction(Direction::Both {
                    vertical: Scrollbar::new(),
                    horizontal: Scrollbar::new(),
                })
                .on_scroll(Message::SyncPianoRoll);

            row![canvas_header, track_canvas].into()
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
