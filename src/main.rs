mod ui {
    pub mod common;
    pub mod div;
    pub mod layout;
    pub mod root;
    pub mod row;
    pub mod text;
}

use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};
use ui::common::{Base, Length};
use ui::div::Div;
use ui::layout::Layout;
use ui::root::Root;
use ui::text::Text;

use crate::ui::common::Direction;

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Rust UI Example")
        .build();

    let div1 = Div::new(
        Rc::new(RefCell::new(Text::new("Hello, world!", 32))),
        (Length::FILL, Length::FILL),
        Color::DARKGRAY,
    );
    let div2 = Div::new(
        Rc::new(RefCell::new(Text::new("This is a test.", 32))),
        (Length::FILL, Length::FILL),
        Color::LIGHTGRAY,
    );

    let row = Layout::new(
        vec![div1, div2],
        (Length::FILL, Length::FILL),
        Color::RED,
        Direction::Row,
    );
    let mut root = Root::new(row, (800, 600));
    root.pass_1((0, 0));
    root.pass_2((0, 0));
    root.debug_dims();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        root.draw(&mut d);
    }
}
