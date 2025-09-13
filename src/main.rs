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
    let el = test();

    let el2 = Layout::get_row_builder()
        .children(vec![
            Layout::get_row_builder()
                .children(vec![Text::new("Hello", 24)])
                .bg_color(Color::LIGHTBLUE)
                .dim((Length::FIT, Length::FIT))
                .build(),
        ])
        .bg_color(Color::DARKGRAY)
        .dim((Length::FIT, Length::FIT))
        .padding((20, 20, 20, 20))
        .build();

    let mut root = Root::new(el, (1000, 1000));

    root.pass_1((0, 0));
    root.pass_2((0, 0));
    root.debug_dims(0);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::YELLOW);
        root.draw(&mut d);
        // draw_grid(&mut d, 1000, 1000, 50);
    }
}

fn test() -> Rc<RefCell<dyn Base>> {
    let text1 = Text::new("Hello", 24);
    let text2 = Text::new("World!", 24);
    let text3 = Text::new("This is a test.", 24);
    let text4 = Text::new("Of the emergency", 24);
    let text5 = Text::new("Broadcast system.", 24);

    let row1 = Layout::get_row_builder()
        .children(vec![text1, text2])
        .bg_color(Color::DARKGRAY)
        .padding((10, 10, 10, 10))
        .gap(10)
        .flex(2.0)
        .build();

    let boxes = vec![text3, text4, text5]
        .iter()
        .enumerate()
        .map(|(i, t)| {
            Layout::get_row_builder()
                .children(vec![t.clone()])
                .bg_color(Color::BLUE)
                .dim((Length::FIT, Length::FILL))
                .padding((5, 5, 5, 5))
                .gap(5)
                .flex((i + 1) as f32) // Example flex value
                .build() as Rc<RefCell<dyn Base>>
        })
        .collect::<Vec<_>>();

    let row2 = Layout::get_col_builder()
        .children(boxes)
        .bg_color(Color::RED)
        .padding((10, 10, 10, 10))
        .gap(10)
        .flex(1.0)
        .main_align(ui::common::Alignment::Start)
        .cross_align(ui::common::Alignment::Center)
        .build();

    let col = Layout::get_col_builder()
        .children(vec![row1, row2])
        .bg_color(Color::GRAY)
        .padding((20, 20, 50, 20))
        .gap(20)
        .build();

    col
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
        return row_builder.clone().children(children).build();
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

fn draw_grid(draw_handle: &mut RaylibDrawHandle, max_x: i32, max_y: i32, gap: i32) {
    let mut x = 0;
    let mut y = 0;
    while x < max_x {
        draw_handle.draw_line(x, 0, x, max_y, Color::PINK);
        x += gap;
    }
    while y < max_y {
        draw_handle.draw_line(0, y, max_x, y, Color::PINK);
        y += gap;
    }
}
