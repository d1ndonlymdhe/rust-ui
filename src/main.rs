mod ui {
    pub mod common;
    pub mod layout;
    pub mod raw_text;
    pub mod root;
    // pub mod text_input;
    pub mod text_input;
    pub mod text_layout;
}

use lazy_static::lazy_static;
use raylib::prelude::*;
use std::sync::{Arc, Mutex};
use std::vec;
use ui::common::{Length, MouseEvent};
use ui::root::UIRoot;

use crate::ui::common::{Alignment, Component, def_key_handler};
use crate::ui::layout::Layout;
use crate::ui::text_input::TextInput;
use crate::ui::text_layout::TextLayout;

lazy_static! {
    static ref CHAT_STATE: Arc<Mutex<ChatState>> = Arc::new(Mutex::new(ChatState::new()));
}

fn main() {
    {
        CHAT_STATE.lock().unwrap().seed_users();
    }
    {
        CHAT_STATE.lock().unwrap().seed_messages();
    }
    UIRoot::start(Box::new(||{chat_layout()}), (1000,1000), "HI!");
}

#[derive(Clone)]
struct ChatUser {
    id: String,
    name: String,
}

#[derive(Clone)]
struct ChatMessage {
    content: String,
    sender_id: String,
    receiver_id: String,
}

struct ChatState {
    users: Vec<ChatUser>,
    messages: Vec<ChatMessage>,
    my_id: String,
    current_user_id: String,
    draft_message: String,
    show_delete_user_popup: Option<String>,
}

impl ChatState {
    fn new() -> Self {
        Self {
            users: vec![],
            messages: vec![],
            current_user_id: String::new(),
            my_id: "0".to_string(),
            draft_message: String::from("Hi!"),
            show_delete_user_popup: None,
        }
    }

    fn add_user(&mut self, id: &str, name: &str) {
        self.users.push(ChatUser {
            id: id.to_string(),
            name: name.to_string(),
        });
    }

    fn add_message(&mut self, content: &str, sender_id: &str, receiver_id: &str) {
        self.messages.push(ChatMessage {
            content: content.to_string(),
            sender_id: sender_id.to_string(),
            receiver_id: receiver_id.to_string(),
        });
    }

    fn seed_users(&mut self) {
        self.add_user("0", "Me");
        self.add_user("1", "Alice");
        self.add_user("2", "Bob");
        self.add_user("3", "Charlie");
        self.add_user("4", "David");
        self.add_user("5", "Eve");
        self.current_user_id = "1".to_string(); // Assume Alice is the active chat
    }

    fn seed_messages(&mut self) {
        //Alice
        for _ in 0..3 {
            self.add_message("Hello Alice!", "0", "1");
            self.add_message("Hi! How are you?", "1", "0");
            self.add_message("I'm good, thanks! And you?", "0", "1");
            self.add_message(
                "Doing well, just working on a project.\n Hello There new line",
                "1",
                "0",
            );
            self.add_message("That's great to hear!", "0", "1");
            self.add_message("What about you?", "1", "0");
            self.add_message("Same here, just busy with work.", "0", "1");
            self.add_message("We should catch up sometime.", "1", "0");
            self.add_message("Definitely! Let's plan for it.", "0", "1");
        }

        //Bob
        self.add_message("Hey Bob!", "0", "2");
        self.add_message("Hey! Long time no see.", "2", "0");
        self.add_message("Yeah, it's been a while. How have you been?", "0", "2");
        self.add_message("I've been good, just busy with work. You?", "2", "0");
        self.add_message("Same here. We should grab coffee sometime.", "0", "2");
        self.add_message("Sounds good! Let's do it.", "2", "0");
    }

    fn delete_user(&mut self, id: &str) {
        self.users = self
            .users
            .iter()
            .filter(|user| user.id != id)
            .map(|user| user.clone())
            .collect::<Vec<_>>();
        self.messages = self
            .messages
            .iter()
            .filter(|msg| msg.receiver_id != id && msg.sender_id != id)
            .map(|msg| msg.clone())
            .collect::<Vec<_>>();
    }
    fn get_user(&self, id: &str) -> Option<ChatUser> {
        for user in self.users.iter() {
            if user.id == id {
                return Some(user.clone());
            }
        }
        return None;
    }

    fn get_current_messages(&self) -> Vec<&ChatMessage> {
        self.messages
            .iter()
            .filter(|msg| {
                msg.sender_id == self.current_user_id || msg.receiver_id == self.current_user_id
            })
            .collect()
    }

    fn set_user_to_delete(&mut self, id: &str) {
        self.show_delete_user_popup = Some(id.to_string());
    }
    fn clear_user_to_delete(&mut self) {
        self.show_delete_user_popup = None;
    }
}

fn users_header() -> Component {
    Layout::get_row_builder()
        .children(vec![
            TextLayout::get_builder()
                .content("Users:")
                .font_size(24)
                .bg_color(Color {
                    r: 100,
                    g: 100,
                    b: 255,
                    a: 255,
                })
                .dim((Length::FIT, Length::FIT))
                .padding((10, 10, 10, 10))
                .build() as Component,
        ])
        .dim((Length::FILL, Length::FIT))
        .main_align(Alignment::Center)
        .bg_color(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        })
        .build() as Component
}

fn users_component() -> Vec<Component> {
    let (users_to_display, my_id, current_user_id) = {
        let chat_state = CHAT_STATE.lock().unwrap();
        let users = chat_state.users.clone();
        let my_id = chat_state.my_id.clone();
        let current_user_id = chat_state.current_user_id.clone();
        (users, my_id, current_user_id)
    };

    users_to_display
        .iter()
        .filter(|user| user.id != my_id)
        .map(|user| {
            let user = user.clone();
            let user_id = user.id.clone();

            Layout::get_row_builder()
                .dim((Length::FILL, Length::FIXED(40)))
                .gap(10)
                .on_click(Box::new(move |_mouse_event: MouseEvent| {
                    CHAT_STATE.lock().unwrap().current_user_id = user_id.clone();
                    true
                }))
                .children(vec![
                    TextLayout::get_builder()
                        .content(&user.name)
                        .dim((Length::FILL, Length::FILL))
                        .bg_color(if current_user_id == user.id {
                            Color::LIGHTGREEN
                        } else {
                            Color::LIGHTGRAY
                        })
                        // .padding((10, 0, 10, 0))
                        .cross_align(Alignment::Start)
                        .main_align(Alignment::Center)
                        .flex(7.0)
                        .build(),
                    Layout::get_col_builder()
                        .children(vec![
                            TextLayout::get_builder()
                                .cross_align(Alignment::Center)
                                .main_align(Alignment::Center)
                                .content("\\/")
                                .font_size(10)
                                .build(),
                            TextLayout::get_builder()
                                .cross_align(Alignment::Center)
                                .main_align(Alignment::Center)
                                .content("/\\")
                                .font_size(10)
                                .build(),
                        ])
                        .cross_align(Alignment::Center)
                        .main_align(Alignment::Center)
                        .on_click({
                            Box::new(move |_| {
                                let mut state = CHAT_STATE.lock().unwrap();
                                state.set_user_to_delete(&user.id);
                                false
                            })
                        })
                        .flex(3.0)
                        .dim((Length::FILL, Length::FILL))
                        .bg_color(Color::RED)
                        .build(),
                ])
                .build() as Component
        })
        .collect::<Vec<_>>()
}

fn message_component(content: String, is_current_user: bool, idx: usize) -> Component {
    Layout::get_col_builder()
        .children(vec![
            TextLayout::get_builder()
                .content(&content)
                .font_size(20)
                .bg_color(if is_current_user {
                    Color::LIGHTGREEN
                } else {
                    Color::SLATEBLUE
                })
                .cross_align(Alignment::Start)
                .border_width(2)
                .main_align(Alignment::Center)
                .dim((Length::FIT, Length::FIT))
                .dbg_name(&format!("MSG {}", idx))
                .padding((5, 2, 5, 2))
                .font_size(20)
                .build(),
        ])
        .overflow_y(false)
        .bg_color(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        })
        .dim((Length::FILL, Length::FIT))
        .cross_align(if is_current_user {
            Alignment::Start
        } else {
            Alignment::End
        })
        .build() as Component
}

fn input_box_component() -> Component {
    let draft_message = {
        let chat_state = CHAT_STATE.lock().unwrap();
        chat_state.draft_message.clone()
    };
    let builder = TextInput::get_builder();
    let builder = builder
        .content(&draft_message)
        .dbg_name("TEXT_INPUT")
        .font_size(20)
        .padding((10, 0, 10, 0))
        .main_align(Alignment::Center)
        .on_key(Box::new(move |key_event| {
            if key_event.key.is_some_and(|v| v == KeyboardKey::KEY_ENTER) {
                let mut chat_state = CHAT_STATE.lock().unwrap();
                let content = chat_state.draft_message.clone();
                if content.trim().is_empty() {
                    return true;
                }
                let current_user_id = chat_state.current_user_id.clone();
                let my_id = chat_state.my_id.clone();
                chat_state.add_message(&content, &my_id, &current_user_id);
                chat_state.draft_message.clear();
            }
            let mut chat_state = CHAT_STATE.lock().unwrap();
            def_key_handler(key_event, &mut chat_state.draft_message);
            true
        }))
        .bg_color(Color::LIGHTGRAY)
        .dim((Length::FILL, Length::FILL))
        .flex(8.0)
        .build();
    builder
}

fn send_button_component() -> Component {
    TextLayout::get_builder()
        .content("Send")
        .font_size(20)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .flex(2.0)
        .on_click(Box::new(move |_mouse_event| {
            let mut chat_state = CHAT_STATE.lock().unwrap();
            let content = chat_state.draft_message.clone();
            if content.trim().is_empty() {
                return true;
            }
            let current_user_id = chat_state.current_user_id.clone();
            let my_id = chat_state.my_id.clone();
            chat_state.add_message(&content, &my_id, &current_user_id);
            chat_state.draft_message.clear();
            true
        }))
        .build()
}

fn messages_component() -> Vec<Component> {
    let messages_data = {
        let chat_state = CHAT_STATE.lock().unwrap();
        let messages = chat_state.get_current_messages();
        messages
            .iter()
            .map(|msg| {
                let is_current_user = msg.sender_id == chat_state.current_user_id;
                (msg.content.clone(), is_current_user)
            })
            .collect::<Vec<_>>()
    };

    messages_data
        .iter()
        .enumerate()
        .map(|(idx, (content, is_current_user))| {
            message_component(content.clone(), *is_current_user, idx)
        })
        .collect::<Vec<_>>()
}

fn input_row_component() -> Component {
    let input_box = input_box_component();
    let send_button = send_button_component();

    Layout::get_row_builder()
        .children(vec![input_box, send_button])
        .dim((Length::FILL, Length::FILL))
        .flex(1f32)
        .build() as Component
}

fn left_sidebar_component() -> Component {
    let header = users_header();
    let mut children = vec![header];
    let users = users_component();
    children.extend(users);
    Layout::get_col_builder()
        .children(children)
        .dim((Length::FILL, Length::FILL))
        .cross_align(Alignment::Center)
        .bg_color(Color::BISQUE)
        .padding((10, 5, 10, 5))
        .dbg_name("LEFT_SIDEBAR")
        .gap(5)
        .flex(1.0)
        .build()
}

fn chat_area_component() -> Component {
    let children = messages_component();
    let messages = Layout::get_col_builder()
        .dbg_name("CHAT_AREA")
        .children(vec![
            Layout::get_col_builder()
                // .bg_color(Color::BEIGE)
                .children(children)
                .gap(2)
                .build(),
        ])
        .flex(19f32)
        .build();
    let input_row = input_row_component();

    Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .overflow_y(false)
        .flex(2.0)
        .children(vec![messages, input_row])
        .build()
}

fn delete_user_popup() -> Component {
    println!("Rendering popup");
    let user_to_delete = {
        let mut state = CHAT_STATE.lock().unwrap();
        let del_user_id = &state.show_delete_user_popup;
        if let Some(del_id) = del_user_id {
            let u = state.get_user(&del_id);
            if u.is_none() {
                state.clear_user_to_delete();
            }
            u
        } else {
            state.clear_user_to_delete();
            None
        }
    };

    if user_to_delete.is_none() {
        return Layout::get_row_builder().build();
    }

    let user_to_delete = user_to_delete.unwrap();

    let header = TextLayout::get_builder()
        .content(&format!(
            "Are you sure you want to delete your conversation with {} ?",
            user_to_delete.name
        ))
        .dbg_name("OVERLAY_HEADER")
        .dim((Length::FIT, Length::FIT))
        .build();

    let button_builder = TextLayout::get_builder()
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .dim((Length::FIT_PER(120), Length::FIT_PER(120)))
        .padding((0, 0, 0, 0));

    let buttons = Layout::get_row_builder()
        .gap(20)
        .children(vec![
            button_builder
                .clone()
                .content("YES")
                .bg_color(Color::GREEN)
                .on_click(Box::new(move |_| {
                    let mut state = CHAT_STATE.lock().unwrap();
                    state.delete_user(&user_to_delete.id);
                    false
                }))
                .build(),
            button_builder
                .clone()
                .content("NO")
                .bg_color(Color::RED)
                .on_click(Box::new(|_| {
                    let mut state = CHAT_STATE.lock().unwrap();
                    state.clear_user_to_delete();
                    false
                }))
                .build(),
        ])
        .dim((Length::FIT, Length::FIT))
        .build();

    let container = Layout::get_col_builder()
        .dim((Length::FIT_PER(105), Length::FIT_PER(120)))
        // .padding((0, 12, 0, 10))
        .gap(10)
        .children(vec![header, buttons])
        .bg_color(Color::ROYALBLUE)
        .main_align(Alignment::Start)
        .cross_align(Alignment::Center)
        .on_click(Box::new(|_| false))
        .dbg_name("OVERLAY_HEADER_CONT")
        .build();

    Layout::get_col_builder()
        .set_position(ui::common::Position::Sticky(0, 0))
        .dim((Length::FILL, Length::FILL))
        .dbg_name("OVERLAY")
        .padding((50, 0, 50, 0))
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .on_click(Box::new(|_| {
            let mut state = CHAT_STATE.lock().unwrap();
            state.clear_user_to_delete();
            false
        }))
        .children(vec![container])
        .build()
}

fn chat_layout() -> Component {
    let left_sidebar = left_sidebar_component();
    let chat_area = chat_area_component();

    let mut children = vec![left_sidebar, chat_area];

    let show_popup = {
        let chat_state = CHAT_STATE.lock().unwrap();
        if chat_state.show_delete_user_popup.is_some() {
            true
        } else {
            false
        }
    };
    if show_popup {
        children.push(delete_user_popup());
    }

    Layout::get_row_builder()
        .dim((Length::FILL, Length::FILL))
        .children(children)
        .dbg_name("ROOT_LAYOUT")
        .bg_color(Color::BEIGE)
        .build()
}
