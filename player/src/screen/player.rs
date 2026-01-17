use iced::{
    Element,
    widget::{column, row},
};

pub struct Player {
    midi_file: Option<midi::File>,
}

pub enum Message {}

pub enum Action {
    None,
}

impl Player {
    pub fn update(&mut self, message: Message) -> Action {
        Action::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![].into()
    }

    fn note_names_view(&self) -> Element<'_, Message> {
        for key in 127..0 {
            let note_name = midi::Note::name_from_key(key);
        }

        column![].into()
    }
}
