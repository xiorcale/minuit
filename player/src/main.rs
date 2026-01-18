use iced::{
    Element,
    widget::{canvas, row, scrollable},
};

mod screen;
mod widget;

use crate::widget::MidiProgram;
use crate::widget::NoteHeaderProgram;

struct App {}

struct Message {}

impl App {
    fn new() -> Self {
        App {}
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let note_header_canvas = self.view_note_header_canvas();
        let midi_canvas = self.view_midi_canvas();

        scrollable(row![note_header_canvas, midi_canvas,]).into()
    }

    fn view_midi_canvas(&self) -> Element<'_, Message> {
        scrollable(canvas(MidiProgram::new()).width(3000).height(3200))
            .horizontal()
            .into()
    }

    fn view_note_header_canvas(&self) -> Element<'_, Message> {
        canvas(NoteHeaderProgram::new())
            .width(50)
            .height(3200)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Minuit Player")
        .run()
}
