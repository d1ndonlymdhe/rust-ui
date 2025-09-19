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

use crate::ui::common::{Alignment, KeyEvent};
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
    let test_element = text_test(&(root.clone() as Rc<RefCell<dyn Base>>)); // Pass root reference here

    {
        let binding = root.clone();
        let mut mut_root = binding.borrow_mut();
        mut_root.set_children(vec![test_element]);
        mut_root.pass_1((0, 0));
        mut_root.pass_2((0, 0));
        mut_root.debug_dims(0);
    }

    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        let left_mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        let key = rl.get_key_pressed();
        let shift_down = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
        let mut d = rl.begin_drawing(&thread);

        if key.is_some() {
            println!("Key pressed: {:?} shift_down: {}", key, shift_down);
        }
        let mouse_event = MouseEvent {
            pos: (mouse_pos.x as i32, mouse_pos.y as i32),
            left_button_down: left_mouse_pressed,
        };
        let key_event = KeyEvent { key, shift_down };
        {
            let binding = root.clone();
            let root = binding.borrow();
            root.draw(&mut d);
            root.get_mouse_event_handlers(mouse_event);
            root.get_key_event_handlers(key_event);
        }
        let binding = root.clone();
        let mut mut_root = binding.borrow_mut();
        {
            mut_root.pass_1((0, 0));
            mut_root.pass_2((0, 0));
        }
    }
}

fn text_test(root: &Rc<RefCell<dyn Base>>) -> Rc<RefCell<dyn Base>> {
    let text_builder = TextLayout::get_builder();

    let text1 = text_builder
        .clone()
        .content("Hello World! This is a test of the text layout system.")
        .font_size(24)
        .wrap(true)
        .bg_color(Color::GREEN)
        .dim((Length::FILL, Length::FIT))
        .cross_align(Alignment::Center)
        .flex(1.0)
        .padding((10, 10, 10, 10))
        .build();

    let btn = Layout::get_col_builder()
        .children(vec![
            TextLayout::get_builder()
                .content("Button")
                .font_size(24)
                .bg_color(Color::BLUE)
                .dim((Length::FIT, Length::FIT))
                // .padding((10, 10, 10, 10))
                .build(),
            Layout::get_row_builder()
                .dim((Length::FIT, Length::FIT))
                .bg_color(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                })
                .main_align(Alignment::Center)
                .gap(10)
                .children(vec![
                    TextLayout::get_builder()
                        .content("No Propagate")
                        .font_size(24)
                        .bg_color(Color::PURPLE)
                        .on_click(Box::new(|_mouse_event| {
                            println!("Don't! Propagate!");
                            false
                        }))
                        .wrap(false)
                        .padding((10, 10, 10, 10))
                        .build(),
                    TextInput::get_builder()
                        .content("Hello how are you?")
                        .dbg_name("text_input")
                        .font_size(24)
                        .dim((Length::FIXED(200), Length::FIT))
                        .bg_color(Color::BLACK)
                        .padding((10, 10, 10, 10))
                        .on_click(Box::new(|_mouse_event| false))
                        .build(),
                ])
                .build(),
        ])
        .cross_align(Alignment::Center)
        .gap(20)
        .dim((Length::FILL, Length::FIT))
        .bg_color(Color::GREEN)
        .padding((10, 10, 10, 10))
        .build();

    let div = Layout::get_col_builder()
        .children({
            let mut children = vec![btn.clone() as Rc<RefCell<dyn Base>>];
            // let mut children = vec![];
            let x = (0..10)
                .map(|_| Rc::new(RefCell::new(text1.borrow().clone())) as Rc<RefCell<dyn Base>>)
                .collect::<Vec<_>>();
            children.extend(x);
            children
        })
        .bg_color(Color::DARKGRAY)
        .padding((10, 10, 10, 10))
        .dim((Length::FILL, Length::FILL))
        .gap(10)
        .cross_align(Alignment::Start)
        .dbg_name("test_div")
        .build();

    let root_clone = root.clone();

    btn.borrow_mut().on_click(Box::new(move |_mouse_event| {
        let d = {
            let root_ref = root_clone.borrow();
            root_ref.get_by_id("test_div")
        };
        if d.is_some() {
            let d = d.unwrap();
            d.borrow_mut()
                .add_child(Rc::new(RefCell::new(text1.borrow().clone())) as Rc<RefCell<dyn Base>>)
        }
        false
    }));

    div
}
