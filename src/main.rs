mod ui {
    pub mod common;
    pub mod layout;
    pub mod root;
    pub mod text;
}

use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};
use ui::common::{Base, Length};
use ui::layout::Layout;
use ui::root::Root;
use ui::text::Text;

use crate::ui::common::{Alignment, Direction};

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Rust UI Example")
        .build();

    let div1 = Layout::new(
        vec![Rc::new(RefCell::new(Text::new("Hello, world!", 32)))],
        (Length::FILL, Length::FIT),
        Color::DARKGRAY,
        Direction::Column,
        Alignment::Start,
        (0, 0, 0, 0),
        0,
    );
    let div2 = Layout::new(
        vec![Rc::new(RefCell::new(Text::new("This is a test.", 32)))],
        (Length::FILL, Length::FILL),
        Color::LIGHTGRAY,
        Direction::Column,
        Alignment::Start,
        (0, 0, 0, 0),
        0,
    );
    let div3 = Layout::new(
        vec![Rc::new(RefCell::new(Text::new("Another div.", 32)))],
        (Length::FIT, Length::FIT),
        Color::GREEN,
        Direction::Column,
        Alignment::Start,
        (0, 0, 0, 0),
        0,
    );

    let row = Layout::new(
        vec![div2, div3],
        (Length::FILL, Length::FILL),
        Color::RED,
        Direction::Row,
        Alignment::Start,
        (0, 0, 0, 0),
        10,
    );
    row.borrow_mut().set_dbg_name("dbg_row");

    let col = Layout::new(
        vec![div1, row],
        (Length::FILL, Length::FILL),
        Color::BLACK,
        Direction::Column,
        Alignment::Start,
        (0, 0, 0, 0),
        10,
    );
    let mut root = Root::new(col, (800, 600));
    root.pass_1((0, 0));
    root.pass_2((0, 0));
    root.debug_dims(0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::YELLOW);
        root.draw(&mut d);
    }
}
