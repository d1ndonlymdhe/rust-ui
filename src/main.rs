mod ui {
    pub mod common;
    pub mod layout;
    pub mod raw_text;
    pub mod root;
    pub mod text_input;
    pub mod text_layout;
}

use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;
use ui::common::{Base, Length, MouseEvent};
use ui::raw_text::RawText;
use ui::root::Root;

use crate::ui::common::{keyboard_key_to_char, shift_character, Alignment, KeyEvent};
use crate::ui::layout::Layout;
use crate::ui::text_input::TextInput;
use crate::ui::text_layout::TextLayout;

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Rust UI Example")
        .build();

    let root = Root::new(RawText::new("Loading", 20, (0, 0, 0, 0)), (1000, 1000));
    let chat_state = Rc::new(RefCell::new(ChatState::new()));
    {
        chat_state.borrow_mut().seed_users();
    }
    {
        chat_state.borrow_mut().seed_messages();
    }

    let binding = root.clone();
    // let mut mut_root = binding.borrow_mut();
    while !rl.window_should_close() {
        {
            let chat_layout = chat_layout(&(root.clone()), chat_state.clone());
            let mut mut_root = binding.borrow_mut();
            mut_root.set_children(vec![chat_layout]);
            mut_root.pass_1((0, 0));
            mut_root.pass_2((0, 0));
            // mut_root.debug_dims(0);
        }
        let mouse_pos = rl.get_mouse_position();
        let left_mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        let key = rl.get_key_pressed();
        let shift_down = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
        let mut d = rl.begin_drawing(&thread);
        let mouse_event = MouseEvent {
            pos: (mouse_pos.x as i32, mouse_pos.y as i32),
            left_button_down: left_mouse_pressed,
        };
        let key_event = KeyEvent { key, shift_down };
        {
            let binding = root.clone();
            let mut root = binding.borrow_mut();
            root.draw(&mut d);
            // let mouse_event = MouseEvent {
            //     pos: (260, 970),
            //     left_button_down: true,
            // };
            // let key_event = KeyEvent {
            //     key: Some(KeyboardKey::KEY_A),
            //     shift_down: false,
            // };
            root.get_mouse_event_handlers(mouse_event);
            root.handle_key_event(key_event);
        }
        // {
        //     let mut mut_root = root.borrow_mut();
        //     mut_root.pass_1((0, 0));
        //     mut_root.pass_2((0, 0));
        // }
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
}

impl ChatState {
    fn new() -> Self {
        Self {
            users: vec![],
            messages: vec![],
            current_user_id: String::new(),
            my_id: "0".to_string(),
            draft_message: String::from("Hi!"),
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

        //just for test, mirror message
        if sender_id == self.my_id {
            self.messages.push(ChatMessage {
                content: format!("Echo: {}", content),
                sender_id: receiver_id.to_string(),
                receiver_id: sender_id.to_string(),
            });
        }
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
        self.add_message("Hello Alice!", "0", "1");
        self.add_message("Hi! How are you?", "1", "0");
        self.add_message("I'm good, thanks! And you?", "0", "1");
        self.add_message("Doing well, just working on a project.", "1", "0");
        self.add_message("That's great to hear!", "0", "1");
        self.add_message("What about you?", "1", "0");
        self.add_message("Same here, just busy with work.", "0", "1");
        self.add_message("We should catch up sometime.", "1", "0");
        self.add_message("Definitely! Let's plan for it.", "0", "1");

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
}

fn chat_layout(
    root: &Rc<RefCell<Root>>,
    root_chat_state: Rc<RefCell<ChatState>>,
) -> Rc<RefCell<dyn Base>> {
    let borrowed_root_chat_state = root_chat_state.borrow();

    let root_div = Layout::get_row_builder()
        .dim((Length::FILL, Length::FILL))
        .children(vec![
            Layout::get_col_builder()
                .children({
                    let header = Layout::get_row_builder()
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
                                .build() as Rc<RefCell<dyn Base>>,
                        ])
                        .dim((Length::FILL, Length::FIT))
                        .main_align(Alignment::Center)
                        .bg_color(Color {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 0,
                        })
                        .build() as Rc<RefCell<dyn Base>>;

                    let mut children = vec![header];
                    // let users = borrowed_root_chat_state.users;
                    let users = borrowed_root_chat_state
                        .users
                        .iter()
                        .filter(|user| user.id != root_chat_state.borrow().my_id)
                        .map(|user| {
                            let user = user.clone();
                            TextLayout::get_builder()
                                .content(&user.name)
                                .font_size(20)
                                .bg_color(Color::LIGHTBLUE)
                                .dim((Length::FILL, Length::FIXED(40)))
                                .padding((10, 10, 10, 10))
                                .on_click({
                                    let on_click = {
                                        let closure_chat_state = root_chat_state.clone();
                                        Box::new(move |_mouse_event: MouseEvent| {
                                            closure_chat_state.borrow_mut().current_user_id =
                                                user.id.clone();
                                            true
                                        })
                                    };
                                    on_click
                                })
                                .build() as Rc<RefCell<dyn Base>>
                        })
                        .collect::<Vec<_>>();

                    children.extend(users);
                    children
                })
                .dim((Length::FILL, Length::FILL))
                .padding((10, 5, 10, 5))
                .bg_color(Color::RED)
                .gap(5)
                .flex(2.5)
                .build(),
            Layout::get_col_builder()
                .dim((Length::FILL, Length::FILL))
                .bg_color(Color::BLUE)
                .flex(7.5)
                .main_align(Alignment::End)
                .children({
                    let chat_state = root_chat_state.borrow();
                    let messages = chat_state.get_current_messages();
                    let mut messages = messages
                        .iter()
                        .map(|msg| {
                            let is_current_user = msg.sender_id == chat_state.current_user_id;
                            Layout::get_col_builder()
                                .children(vec![
                                    TextLayout::get_builder()
                                        .content(&msg.content)
                                        .font_size(20)
                                        .bg_color(if is_current_user {
                                            Color::LIGHTGREEN
                                        } else {
                                            Color::SLATEBLUE
                                        })
                                        .dim((Length::PERCENT(50), Length::FIT))
                                        .padding((10, 10, 10, 10))
                                        .font_size(20)
                                        .build(),
                                ])
                                .bg_color(Color {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 0,
                                })
                                .dim((Length::FILL, Length::FIT))
                                .cross_align(if is_current_user {
                                    Alignment::End
                                } else {
                                    Alignment::Start
                                })
                                .build() as Rc<RefCell<dyn Base>>
                        })
                        .collect::<Vec<_>>();

                    let input_box = TextInput::get_builder()
                        .content("Type a message...")
                        .font_size(20)
                        .bg_color(Color::LIGHTGRAY)
                        .dim((Length::FILL, Length::FIXED(40)))
                        .flex(8.0)
                        .build();
                    let closure_chat_state = root_chat_state.clone();
                    messages.push(
                        Layout::get_row_builder()
                            .children(vec![
                                {
                                    let builder = TextInput::get_builder();
                                    let builder = {
                                        builder.content(closure_chat_state
                                            .borrow()
                                            .draft_message
                                            .as_str())
                                    };
                                    builder.dbg_name("TEXT_INPUT")
                                    .font_size(20)
                                    .on_key(Box::new(move |key_event| {
                                        let mut chat_state = closure_chat_state.borrow_mut();
                                        match key_event.key {
                                            Some(KeyboardKey::KEY_BACKSPACE) => {
                                                chat_state.draft_message.pop();
                                            }
                                            Some(key) => {
                                                if let Some(c) = keyboard_key_to_char(key) {
                                                    let mut c = c;
                                                    if key_event.shift_down {
                                                        c = shift_character(c);
                                                    }
                                                    chat_state.draft_message.push(c);
                                                }
                                            }
                                            None => {}
                                        }
                                        true
                                    }))
                                    .bg_color(Color::LIGHTGRAY)
                                    .dim((Length::FILL, Length::FIXED(40)))
                                    .flex(8.0)
                                    .build()
                                },
                                TextLayout::get_builder()
                                    .content("Send")
                                    .font_size(20)
                                    .bg_color(Color::DARKGRAY)
                                    .dim((Length::FILL, Length::FIXED(40)))
                                    .main_align(Alignment::Center)
                                    .cross_align(Alignment::Center)
                                    .flex(2.0)
                                    // .on_click(Box::new(move |_mouse_event| {
                                    //     let input_box_borrowed = input_box.borrow();
                                    //     let content = input_box_borrowed.get_content();
                                    //     if content.trim().is_empty() {
                                    //         return true;
                                    //     }
                                    //     let current_user_id =
                                    //         closure_chat_state.borrow().current_user_id.clone();
                                    //     let my_id = closure_chat_state.borrow().my_id.clone();
                                    //     closure_chat_state.borrow_mut().add_message(
                                    //         &content,
                                    //         &my_id,
                                    //         &current_user_id,
                                    //     );
                                    //     input_box_borrowed.set_content("");
                                    //     true
                                    // }))
                                    .build(),
                            ])
                            .dim((Length::FILL, Length::FIT))
                            .build() as Rc<RefCell<dyn Base>>,
                    );
                    messages
                })
                .build(),
        ])
        .build();
    root_div
}
