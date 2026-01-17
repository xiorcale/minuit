use iced::{
    Element,
    widget::{
        canvas, scrollable,
        scrollable::{Direction, Scrollbar},
    },
};

mod screen;
mod widget;

use crate::widget::MidiProgram;

struct App {}

struct Message {}

impl App {
    fn new() -> Self {
        App {}
    }

    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let midi_program = MidiProgram::new();
        let canvas = canvas(midi_program).width(3000).height(3000);

        scrollable(canvas)
            .direction(Direction::Both {
                vertical: Scrollbar::default(),
                horizontal: Scrollbar::default(),
            })
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Minuit Player")
        .run()
}
