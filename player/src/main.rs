use iced::Element;
use iced::Task;

mod screen;
mod widget;

use crate::screen::Player;
use crate::screen::player;

struct App {
    player: Player,
}

#[derive(Debug, Clone)]
enum Message {
    PlayerMessage(player::Message),
}

impl App {
    fn new() -> Self {
        App {
            player: Player::new(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlayerMessage(msg) => self
                .player
                .update(msg)
                .map(|msg| Message::PlayerMessage(msg)),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.player.view().map(|msg| Message::PlayerMessage(msg))
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Minuit Player")
        .run()
}
