mod ui {
    pub mod common;
    pub mod layout;
    pub mod root;
    pub mod text;
}

use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;
use ui::common::{Base, Length};
use ui::root::Root;
use ui::text::Text;

use crate::ui::common::{Alignment, Direction};
use crate::ui::layout::LayoutProps;

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Rust UI Example")
        .build();

    let layout_builder = LayoutProps::new();
    let col_builder = layout_builder
        .clone()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::from_hex("FFFFFF").unwrap().alpha(1f32))
        .direction(Direction::Column)
        .align(Alignment::Start)
        .padding((0, 0, 0, 0))
        .gap(0);

    let row_builder = layout_builder
        .clone()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::from_hex("FFFFFF").unwrap().alpha(1f32))
        .direction(Direction::Row)
        .align(Alignment::Start)
        .padding((0, 0, 0, 0))
        .gap(0);

    // Create the spiral design with numbered sections
    // Start from the innermost section and work outwards

    // Section 8 - innermost
    let section_8 = col_builder
        .clone()
        .children(vec![Text::new("8", 48)])
        .bg_color(Color::PINK)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 7 - divide section 8's container into top-bottom, put 7 on top
    let section_7_8 = col_builder
        .clone()
        .children(vec![
            col_builder
                .clone()
                .children(vec![Text::new("7", 48)])
                .bg_color(Color::LIME)
                .dim((Length::FILL, Length::FILL))
                .build(),
            section_8,
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 6 - divide into left-right, put 6 on left
    let section_6_7_8 = row_builder
        .clone()
        .children(vec![
            col_builder
                .clone()
                .children(vec![Text::new("6", 48)])
                .bg_color(Color::ORANGE)
                .dim((Length::FILL, Length::FILL))
                .build(),
            section_7_8,
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 5 - divide into top-bottom, put 5 on bottom
    let section_5_6_7_8 = col_builder
        .clone()
        .children(vec![
            section_6_7_8,
            col_builder
                .clone()
                .children(vec![Text::new("5", 48)])
                .bg_color(Color::SKYBLUE)
                .dim((Length::FILL, Length::FILL))
                .build(),
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 4 - divide into left-right, put 4 on right
    let section_4_5_6_7_8 = row_builder
        .clone()
        .children(vec![
            section_5_6_7_8,
            col_builder
                .clone()
                .children(vec![Text::new("4", 48)])
                .bg_color(Color::YELLOW)
                .dim((Length::FILL, Length::FILL))
                .build(),
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 3 - divide into top-bottom, put 3 on top
    let section_3_4_5_6_7_8 = col_builder
        .clone()
        .children(vec![
            col_builder
                .clone()
                .children(vec![Text::new("3", 48)])
                .bg_color(Color::PURPLE)
                .dim((Length::FILL, Length::FILL))
                .build(),
            section_4_5_6_7_8,
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 2 - divide into left-right, put 2 on left
    let section_2_3_4_5_6_7_8 = row_builder
        .clone()
        .children(vec![
            col_builder
                .clone()
                .children(vec![Text::new("2", 48)])
                .bg_color(Color::GREEN)
                .dim((Length::FILL, Length::FILL))
                .build(),
            section_3_4_5_6_7_8,
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    // Section 1 - divide into top-bottom, put 1 on top
    let col = col_builder
        .clone()
        .children(vec![
            col_builder
                .clone()
                .children(vec![Text::new("1", 48)])
                .bg_color(Color::RED)
                .dim((Length::FILL, Length::FILL))
                .build(),
            section_2_3_4_5_6_7_8,
        ])
        .gap(2)
        .bg_color(Color::DARKGRAY)
        .dim((Length::FILL, Length::FILL))
        .build();

    let el = make_spiral(1, 5);
    let mut root = Root::new(el, (1000, 1000));
    root.pass_1((0, 0));
    root.pass_2((0, 0));
    root.debug_dims(0);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::YELLOW);
        root.draw(&mut d);
    }
}

fn make_spiral(curr_depth: usize, max_depth: usize) -> Rc<RefCell<dyn Base>> {
    let layout_builder = LayoutProps::new();
    let col_builder = layout_builder
        .clone()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::from_hex("FFFFFF").unwrap().alpha(1f32))
        .direction(Direction::Column)
        .align(Alignment::Start)
        .padding((0, 0, 0, 0))
        .gap(0);

    let row_builder = layout_builder
        .clone()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::from_hex("FFFFFF").unwrap().alpha(1f32))
        .direction(Direction::Row)
        .align(Alignment::Start)
        .padding((0, 0, 0, 0))
        .gap(0);

    if curr_depth == max_depth {
        return Text::new(&format!("{}", curr_depth), 12);
    }

    let child = make_spiral(curr_depth + 1, max_depth);
    if curr_depth % 2 == 0 {
        // even - row
        return row_builder
            .clone()
            .children(vec![
                col_builder
                    .clone()
                    .children(vec![Text::new(&format!("{}", curr_depth), 12)])
                    .bg_color(Color::ORANGE)
                    .dim((Length::FILL, Length::FILL))
                    .build(),
                child,
            ])
            .gap(2)
            .bg_color(Color::DARKGRAY)
            .dim((Length::FILL, Length::FILL))
            .build();
    } else {
        // odd - col
        return col_builder
            .clone()
            .children(vec![
                child,
                col_builder
                    .clone()
                    .children(vec![Text::new(&format!("{}", curr_depth), 12)])
                    .bg_color(Color::LIME)
                    .dim((Length::FILL, Length::FILL))
                    .build(),
            ])
            .gap(2)
            .bg_color(Color::DARKGRAY)
            .dim((Length::FILL, Length::FILL))
            .build();
    }
}
