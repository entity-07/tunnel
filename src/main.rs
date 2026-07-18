use std::time::SystemTime;

use iced::Element;
use iced::Length;
use iced::Theme;
use iced::color;
use iced::widget;

#[derive(Default)]
struct App {
    screen: Screen,
    contacts: Vec<User>,
    pinging: bool,
    current_input: String,
    focused_conversation: Option<usize>,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    saved_as: String,
    id: String,
    trusted: bool,
    history: ChatHistory,
}

#[derive(Debug, Clone)]
struct ChatHistory {
    history: Vec<Chat>,
}

#[derive(Debug, Clone)]
struct Chat {
    content: String,
    time: SystemTime,
}

#[derive(Debug, Default, Clone)]
enum Screen {
    #[default]
    Home,
    Search,
    Chats,
}

#[derive(Clone, Debug)]
enum Message {
    SwitchScreen(Screen),
    SelectConversation(usize),
    SendChat,
    InputChanged(String),
    Ping,
}

impl App {
    fn render_nav_panel(&self) -> Element<'_, Message> {
        return widget::column![
            widget::button(widget::text!("Home")).on_press(Message::SwitchScreen(Screen::Home)),
            widget::button(widget::text!("Signal Search"))
                .on_press(Message::SwitchScreen(Screen::Search)),
            widget::button(widget::text!("Chats")).on_press(Message::SwitchScreen(Screen::Chats)),
        ]
        .spacing(10)
        .padding(10)
        .into();
    }

    fn show_chats_or_history(&self) -> Element<'_, Message> {
        if let Some(idx) = self.focused_conversation {
            self.render_history(idx)
        } else {
            match self.screen {
                Screen::Home => self.render_home(),
                Screen::Search => self.render_search(),
                Screen::Chats => self.render_chats(),
            }
        }
    }

    fn render_history(&self, idx: usize) -> Element<'_, Message> {
        let to_render = &self.contacts[idx];
        return widget::container(widget::column![
            widget::column(
                to_render
                    .history
                    .history
                    .iter()
                    .map(|msg| widget::text(&msg.content).into()),
            ),
            widget::text_input("smth to say?", &self.current_input)
                .on_input(Message::InputChanged)
                .on_submit(Message::SendChat)
        ])
        .into();
    }

    fn view(&self) -> Element<'_, Message> {
        return widget::row![self.render_nav_panel(), self.show_chats_or_history()].into();
    }

    fn render_home(&self) -> Element<'_, Message> {
        return widget::container(widget::column![widget::svg(
            "assets/icons/home-svgrepo-com.svg"
        )])
        .center(Length::Fill)
        .into();
    }

    fn render_search(&self) -> Element<'_, Message> {
        return widget::container(widget::column![
            widget::text("THIS IS THE SIGNAL SEARCH").color(color!(0x0000ff)),
            widget::button(widget::text("[Ping]")).on_press(Message::Ping)
        ])
        .center(Length::Fill)
        .into();
    }

    fn render_chats(&self) -> Element<'_, Message> {
        return widget::container(
            widget::keyed_column(self.contacts.iter().enumerate().map(|(i, contact)| {
                (
                    i,
                    widget::button(widget::text(&contact.name))
                        .on_press(Message::SelectConversation(i))
                        .into(),
                )
            }))
            .spacing(10),
        )
        .center(Length::Fill)
        .into();
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchScreen(screen) => {
                println!("Going to screen: {:#?}", screen);
                self.screen = screen;
                println!("{:#?}", self.screen);
                self.focused_conversation = None;
            }
            Message::SendChat => {
                if !self.current_input.is_empty() {
                    let Some(idx) = self.focused_conversation else {
                        return;
                    };
                    let recipient = &mut self.contacts[idx];
                    recipient.history.history.push(Chat {
                        content: std::mem::take(&mut self.current_input),
                        time: SystemTime::now(),
                    })
                } else {
                    ()
                }
            }
            Message::InputChanged(message) => {
                self.current_input = message;
            }
            Message::SelectConversation(idx) => {
                println!("Selecting convo {:#?}", idx);
                self.focused_conversation = Some(idx);
                println!("Current convo {:#?}", self.focused_conversation);
            }
            Message::Ping => {
                self.pinging = true;
                let user1 = User {
                    name: String::from("json"),
                    saved_as: String::new(),
                    id: "0001".to_owned(),
                    trusted: true,
                    history: ChatHistory {
                        history: vec![
                            Chat {
                                content: String::from("hi"),
                                time: SystemTime::now(),
                            },
                            Chat {
                                content: String::from("yo"),
                                time: SystemTime::now(),
                            },
                        ],
                    },
                };
                let user2 = User {
                    name: String::from("dvid"),
                    saved_as: String::new(),
                    id: "0002".to_owned(),
                    trusted: true,
                    history: ChatHistory {
                        history: Vec::new(),
                    },
                };
                self.contacts.push(user1);
                println!("{:#?}", self.contacts);
                self.contacts.push(user2);
                println!("{:#?}", self.contacts);
                self.pinging = false;
            }
        }
    }
    fn new() -> Self {
        return Self {
            screen: Screen::Home,
            contacts: Vec::new(),
            pinging: false,
            current_input: String::new(),
            focused_conversation: None,
        };
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(Theme::GruvboxDark)
        .run()
}
