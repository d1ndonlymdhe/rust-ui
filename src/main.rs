mod ui {
    pub mod common;
    pub mod layout;
    pub mod raw_text;
    pub mod root;
    pub mod text_input;
    pub mod text_layout;
}

use lazy_static::lazy_static;
use raylib::prelude::*;
use std::sync::{Arc, Mutex};
use std::vec;
use ui::common::{Length, MouseEvent};
use ui::raw_text::RawText;
use ui::root::Root;

use crate::ui::common::{Alignment, Component, KeyEvent, ScrollEvent, def_key_handler};
use crate::ui::layout::Layout;
use crate::ui::text_input::TextInput;
use crate::ui::text_layout::TextLayout;

lazy_static! {
    static ref CHAT_STATE: Arc<Mutex<ChatState>> = Arc::new(Mutex::new(ChatState::new()));
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Rust UI Example")
        .build();

    let root = Root::new(
        RawText::new("Loading", 20, (0, 0, 0, 0), Color::BLACK),
        (1000, 1000),
    );
    {
        CHAT_STATE.lock().unwrap().seed_users();
    }
    {
        CHAT_STATE.lock().unwrap().seed_messages();
    }

    let binding = root.clone();
    {
        let chat_layout = chat_layout();
        let mut mut_root = binding.borrow_mut();
        mut_root.set_children(vec![chat_layout]);
        mut_root.pass_1();
        mut_root.pass_2();
        mut_root.pass_overflow();
        mut_root.debug_dims(0);
    }
    rl.set_target_fps(60);
    let mut should_rebuild_ui = true;

    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        let left_mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        let key = rl.get_key_pressed();
        let shift_down = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);

        let ctrl_down = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);

        let mouse_event = MouseEvent {
            pos: (mouse_pos.x as i32, mouse_pos.y as i32),
            left_button_down: left_mouse_pressed,
        };

        let mut d = rl.begin_drawing(&thread);
        let wheel_move = d.get_mouse_wheel_move_v();
        let scroll_y = wheel_move.y;

        let scroll_event = ScrollEvent {
            pos: (mouse_pos.x as i32, mouse_pos.y as i32),
            delta: scroll_y as i32,
        };

        let key_event = KeyEvent {
            key,
            shift_down,
            ctrl_down,
        };
        {
            let binding = root.clone();
            let mut root = binding.borrow_mut();
            let a = root.get_mouse_event_handlers(mouse_event);
            let b = root.handle_key_event(key_event);
            let c = root.get_scroll_event_handler(scroll_event);
            if a || b || c {
                should_rebuild_ui = true;
            }
        }
        if should_rebuild_ui {
            {
                let chat_layout = chat_layout();
                let mut mut_root = binding.borrow_mut();
                mut_root.set_children(vec![chat_layout]);
                mut_root.pass_1();
                mut_root.pass_2();
                mut_root.pass_overflow();
            }
            root.borrow_mut().draw(&mut d);
            should_rebuild_ui = false;
        }
    }
}

#[derive(Clone)]
struct ChatUser {
    id: String,
    name: String,
}

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
    show_delete_user_popup: bool,
}

impl ChatState {
    fn new() -> Self {
        Self {
            users: vec![],
            messages: vec![],
            current_user_id: String::new(),
            my_id: "0".to_string(),
            draft_message: String::from("Hi!"),
            show_delete_user_popup: true,
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

    fn get_current_messages(&self) -> Vec<&ChatMessage> {
        self.messages
            .iter()
            .filter(|msg| {
                msg.sender_id == self.current_user_id || msg.receiver_id == self.current_user_id
            })
            .collect()
    }

    fn toggle_delete_user_popup(&mut self) {
        self.show_delete_user_popup = !self.show_delete_user_popup;
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
            TextLayout::get_builder()
                .content(&user.name)
                .font_size(20)
                .bg_color(if current_user_id == user.id {
                    Color::LIGHTGREEN
                } else {
                    Color::LIGHTGRAY
                })
                .dim((Length::FILL, Length::FIXED(40)))
                .padding((10, 10, 10, 10))
                .on_click({
                    Box::new(move |_mouse_event: MouseEvent| {
                        CHAT_STATE.lock().unwrap().current_user_id = user_id.clone();
                        true
                    })
                })
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
                .dim((Length::FIT, Length::FIT))
                .dbg_name(&format!("MSG {}", idx))
                .padding((10, 10, 10, 10))
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
        .on_key(Box::new(move |key_event| {
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
    let btn = Layout::get_row_builder()
        .children(vec![TextLayout::get_builder().content("Delete").build()])
        .build();
    children.extend(users);
    children.push(btn);
    Layout::get_col_builder()
        .children(children)
        .dim((Length::FILL, Length::FILL))
        .padding((10, 5, 10, 5))
        .bg_color(Color::RED)
        .dbg_name("LEFT_SIDEBAR")
        .gap(5)
        .flex(1f32)
        .build()
}

fn chat_area_component() -> Component {
    let messages = messages_component();

    let messages = Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::BLUE)
        .dbg_name("CHAT_AREA")
        .dim((Length::FILL, Length::PERCENT(100)))
        // .main_align(Alignment::End)
        .children(messages)
        .flex(19f32)
        .build();
    let input_row = input_row_component();

    Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .overflow_y(false)
        .padding((10, 10, 10, 10))
        .bg_color(Color {
            r: 200,
            g: 200,
            b: 200,
            a: 255,
        })
        .flex(3f32)
        .children(vec![messages, input_row])
        .build()
}

fn delete_user_popup() -> Component {
    Layout::get_col_builder()
        .set_position(ui::common::Position::GlobalAbsolute(0, 500))
        .bg_color(Color::BLACK)
        .dim((Length::FILL, Length::FILL))
        .children(vec![
            TextLayout::get_builder()
                .content("POPUP?")
                .font_size(20)
                .bg_color(Color::WHITE)
                .dim((Length::FILL, Length::FIT))
                .padding((10, 10, 10, 10))
                .build(),
        ])
        .build()
}

fn chat_layout() -> Component {
    let left_sidebar = left_sidebar_component();
    let chat_area = chat_area_component();

    let mut children = vec![left_sidebar, chat_area];
    let show_popup;
    {
        let chat_state = CHAT_STATE.lock().unwrap();
        show_popup = chat_state.show_delete_user_popup;
    }
    if show_popup {
        // children.push(delete_user_popup());
    }

    Layout::get_row_builder()
        .dim((Length::FILL, Length::FILL))
        .children(children)
        .build()
}
