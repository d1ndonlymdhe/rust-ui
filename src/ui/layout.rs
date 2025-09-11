use std::{cell::RefCell, rc::Rc};

use raylib::{color::Color, prelude::RaylibDraw};

use crate::ui::common::{Alignment, Base, Direction, Length, tabbed_print};

#[derive(Clone)]
pub struct Layout {
    pub children: Vec<Rc<RefCell<dyn Base>>>,
    pub dim: (Length, Length),
    pub draw_dim: (i32, i32),
    pub pos: (i32, i32),
    pub bg_color: Color,
    pub direction: Direction,
    // Padding (top, right, bottom, left)
    pub padding: (i32, i32, i32, i32),
    pub align: Alignment,
    pub gap: i32,
    pub dbg_name: String,
    pub flex: f32,
}

#[derive(Clone)]
pub struct LayoutProps {
    layout: Layout,
}

impl LayoutProps {
    pub fn new() -> Self {
        Self {
            layout: Layout {
                children: vec![],
                dim: (Length::FILL, Length::FILL),
                draw_dim: (0, 0),
                pos: (0, 0),
                bg_color: Color::WHITE,
                direction: Direction::Row,
                align: Alignment::Start,
                padding: (0, 0, 0, 0),
                gap: 0,
                dbg_name: "".into(),
                flex: 1.0,
            },
        }
    }
    pub fn children(mut self, children: Vec<Rc<RefCell<dyn Base>>>) -> Self {
        self.layout.children = children;
        self
    }
    pub fn dim(mut self, dim: (Length, Length)) -> Self {
        self.layout.dim = dim;
        self
    }
    pub fn bg_color(mut self, color: Color) -> Self {
        self.layout.bg_color = color;
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.direction = direction;
        self
    }
    pub fn align(mut self, align: Alignment) -> Self {
        self.layout.align = align;
        self
    }
    pub fn padding(mut self, padding: (i32, i32, i32, i32)) -> Self {
        self.layout.padding = padding;
        self
    }
    pub fn gap(mut self, gap: i32) -> Self {
        self.layout.gap = gap;
        self
    }
    pub fn dbg_name(mut self, name: &str) -> Self {
        self.layout.dbg_name = name.into();
        self
    }
    pub fn flex(mut self, flex: f32) -> Self {
        self.layout.flex = flex;
        self
    }
    pub fn build(&self) -> Rc<RefCell<Layout>> {
        Rc::new(RefCell::new(self.layout.clone()))
    }
}

impl Layout {
    pub fn get_row_builder() -> LayoutProps {
        LayoutProps::new()
    }
    pub fn get_col_builder() -> LayoutProps {
        LayoutProps::new().direction(Direction::Column)
    }
}

impl Base for Layout {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
    fn draw(&self, draw_handle: &mut raylib::prelude::RaylibDrawHandle) {
        draw_handle.draw_rectangle(
            self.pos.0,
            self.pos.1,
            self.draw_dim.0,
            self.draw_dim.1,
            self.bg_color,
        );
        for child in self.children.iter() {
            let child = child.clone();
            child.borrow_mut().draw(draw_handle);
        }
    }
    fn set_dim(&mut self, parent_dim: (i32, i32)) {
        let (draw_width, draw_height) =
            crate::ui::common::get_draw_dim(self.dim, parent_dim, &self.children, &self.direction);
        self.draw_dim = (draw_width, draw_height);
    }
    fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
    fn pass_1(&mut self, parent_draw_dim: (i32, i32)) {
        let child_len = self.children.len() as i32;
        let total_flex = self
            .children
            .iter()
            .map(|child| child.borrow().get_flex())
            .sum::<f32>();
        for child in self.children.iter() {
            let flex = child.borrow().get_flex();
            match self.direction {
                Direction::Row => {
                    let allowed_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
                    let allowed_width = allowed_width - (self.gap * (child_len - 1));
                    let child_width = f32::round(flex * (allowed_width as f32 / total_flex)) as i32;
                    // let child_width =
                    let child_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
                    child.borrow_mut().set_dim((child_width, child_height));
                    child.borrow_mut().pass_1((child_width, child_height));
                }
                Direction::Column => {
                    let allowed_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
                    let allowed_height = allowed_height - (self.gap * (child_len - 1));
                    let child_height =
                        f32::round(flex * (allowed_height as f32 / total_flex)) as i32;
                    // let child_height = (allowed_height - (self.gap * (child_len - 1))) / child_len;
                    let child_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
                    child.borrow_mut().set_dim((child_width, child_height));
                    child.borrow_mut().pass_1((child_width, child_height));
                }
            }
        }
        self.set_dim(parent_draw_dim);
    }
    fn pass_2(&mut self, parent_pos: (i32, i32)) {
        self.pos = parent_pos;
        let mut next_pos = self.pos;
        next_pos.0 += self.padding.0;
        next_pos.1 += self.padding.1;
        for child in self.children.iter() {
            child.borrow_mut().pass_2(next_pos);
            let (child_width, child_height) = child.borrow().get_draw_dim();
            match self.direction {
                Direction::Row => next_pos.0 += child_width + self.gap,
                Direction::Column => next_pos.1 += child_height + self.gap,
            }
        }
    }
    fn get_flex(&self) -> f32 {
        self.flex
    }
    fn debug_dims(&self, depth: usize) {
        tabbed_print(
            &format!(
                "<layout width={} height={} x={} y={} >",
                self.draw_dim.0, self.draw_dim.1, self.pos.0, self.pos.1
            ),
            depth,
        );
        for (_i, child) in self.children.iter().enumerate() {
            child.borrow().debug_dims(depth + 1);
        }
        tabbed_print("</layout>", depth);
    }
}
