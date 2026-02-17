use crate::widget::canvas_config::{NOTE_HEIGHT, NOTE_WIDTH, NUM_MIDI_NOTES};
use crate::widget::{self, PianoRollRenderer, TrackRenderer};
use iced::widget::{center, container};
use iced::{
    Element,
    Length::{self, Fill},
    Task,
    widget::{
        Id, Scrollable, Space, button, canvas, column,
        operation::{self, AbsoluteOffset},
        row,
        scrollable::{Direction, Scrollbar, Viewport},
    },
};

const PIANO_ROLL_ID: &str = "piano_roll";
const TRACK_RENDERER_ID: &str = "track_renderer";

pub struct Player {
    midi_file: Option<midi::File>,
    track_hpos: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile(String),
    PianoRollScroll(Viewport),
    TrackScroll(Viewport),
}

impl Player {
    pub fn new() -> Self {
        Player {
            midi_file: None,
            track_hpos: 0.0,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile(filepath) => {
                self.midi_file = Some(midi::File::new(filepath));
                Task::none()
            }
            Message::PianoRollScroll(viewport) => operation::scroll_to(
                TRACK_RENDERER_ID,
                AbsoluteOffset {
                    x: self.track_hpos,
                    y: viewport.absolute_offset().y,
                },
            ),
            Message::TrackScroll(viewport) => {
                self.track_hpos = viewport.absolute_offset().x;
                operation::scroll_to(PIANO_ROLL_ID, viewport.absolute_offset())
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(ref midi_file) = self.midi_file {
            let piano_roll_renderer = PianoRollRenderer::new(&midi_file.tracks[1]);
            // let track_renderer = TrackRenderer::new(&midi_file.tracks[1]);

            let (canvas_width, canvas_height) = self.get_canvas_dimension().unwrap();

            let canvas_header =
                Scrollable::new(canvas(piano_roll_renderer).width(50).height(canvas_height))
                    .id(PIANO_ROLL_ID)
                    .direction(Direction::Vertical(Scrollbar::hidden()))
                    .on_scroll(Message::PianoRollScroll);

            // let track_canvas = Scrollable::new(
            //     canvas(track_renderer)
            //         .width(canvas_width)
            //         .height(canvas_height),
            // )
            // .id(TRACK_RENDERER_ID)
            // .direction(Direction::Both {
            //     vertical: Scrollbar::new(),
            //     horizontal: Scrollbar::new(),
            // })
            // .on_scroll(Message::TrackScroll);

            row![container(canvas_header).padding(10) /*track_canvas*/,].into()
        } else {
            let open_file =
                button("click to load file").on_press(Message::OpenFile("./test.mid".to_string()));

            return center(open_file).into();
        }
    }

    fn get_canvas_dimension(&self) -> Option<(u32, u32)> {
        if let Some(ref midi_file) = self.midi_file {
            let track = &midi_file.tracks[1];

            let total_time = track.notes.last()?.start_time + track.notes.last()?.duration;

            let width = total_time / (NOTE_WIDTH as u32);
            let height = track.note_range() as u32 * (NOTE_HEIGHT as u32);

            println!("track len: {}", total_time);

            Some((width, height))
        } else {
            None
        }
    }
}
