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

use crate::ui::layout::Layout;

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Rust UI Example")
        .build();
    let el = make_spiral(0, 10);
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
    let color = Color::new(0, 0, 0, 0);
    let col_builder = Layout::get_col_builder().bg_color(color);
    let row_builder = Layout::get_row_builder().bg_color(color);
    if curr_depth == max_depth {
        if curr_depth % 2 == 0 {
            return row_builder
                .clone()
                .children(vec![Text::new(&format!("{}", curr_depth), 12)])
                .build();
        } else {
            return col_builder
                .clone()
                .children(vec![Text::new(&format!("{}", curr_depth), 12)])
                .build();
        }
    }

    let child = make_spiral(curr_depth + 1, max_depth);
    if curr_depth % 2 == 0 {
        // even - row
        let children = {
            let mut c = vec![
                col_builder
                    .clone()
                    .children(vec![Text::new(&format!("{}", curr_depth), 12)])
                    .build(),
                child,
            ];
            if curr_depth % 4 == 3 || curr_depth % 4 == 2 {
                c.reverse();
            }
            c
        };
        return row_builder
            .clone()
            .children(children)
            .build();
    } else {
        let col_child = row_builder.clone().bg_color(Color::LIME);
        let children = {
            let mut c = vec![
                col_child
                    .children(vec![Text::new(&format!("{}", curr_depth), 12)])
                    .build(),
                child,
            ];
            if curr_depth % 4 == 3 || curr_depth % 4 == 2 {
                c.reverse();
            }
            c
        };

        // odd - col
        return col_builder
            .clone()
            .children(children)
            .gap(2)
            .bg_color(Color::new(0, 255, 0, 255))
            .dim((Length::FILL, Length::FILL))
            .build();
    }
}
