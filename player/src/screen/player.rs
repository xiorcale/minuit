use iced::{
    Element,
    widget::{button, canvas, row, scrollable},
};

use crate::widget::{CanvasTrack, NoteHeaderProgram};

pub struct Player {
    midi_file: Option<midi::File>,
    canvas_track: Option<CanvasTrack>,
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
        Player {
            midi_file: None,
            canvas_track: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::OpenFile(filepath) => {
                self.midi_file = Some(midi::File::new(filepath));

                if let Some(ref midi_file) = self.midi_file {
                    self.canvas_track = Some(CanvasTrack::new(midi_file.tracks[1].clone()));
                }
            }
        }
        Action::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(ref canvas_track) = self.canvas_track {
            let canvas_header = canvas(NoteHeaderProgram::new()).width(50).height(3500);

            let midi_canvas =
                scrollable(canvas(canvas_track).width(3000).height(3500)).horizontal();

            scrollable(row![canvas_header, midi_canvas]).into()
        } else {
            let open_file =
                button("click to load file").on_press(Message::OpenFile("./test.mid".to_string()));

            return row![open_file].into();
        }
    }
}
