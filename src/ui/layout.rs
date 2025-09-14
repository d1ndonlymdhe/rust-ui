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
    pub main_align: Alignment,
    pub cross_align: Alignment,
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
                main_align: Alignment::Start,
                cross_align: Alignment::Start,
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
    pub fn main_align(mut self, align: Alignment) -> Self {
        self.layout.main_align = align;
        self
    }
    pub fn cross_align(mut self, align: Alignment) -> Self {
        self.layout.cross_align = align;
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
        self.draw_dim = (
            draw_width + self.padding.0 + self.padding.3,
            draw_height + self.padding.1 + self.padding.2,
        );
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
                    let child_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
                    child.borrow_mut().set_dim((child_width, child_height));
                    child.borrow_mut().pass_1((child_width, child_height));
                }
                Direction::Column => {
                    let allowed_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
                    let allowed_height = allowed_height - (self.gap * (child_len - 1));
                    let child_height =
                        f32::round(flex * (allowed_height as f32 / total_flex)) as i32;
                    let child_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
                    child.borrow_mut().set_dim((child_width, child_height));
                    child.borrow_mut().pass_1((child_width, child_height));
                }
            }
        }
        self.set_dim(parent_draw_dim);
    }
    fn pass_2(&mut self, passed_pos: (i32, i32)) {
        let mut padding_left = self.padding.0;
        let mut padding_top = self.padding.1;

        let mut cross_paddings = Vec::from(
            (0..self.children.len())
                .map(|_| self.padding.1)
                .collect::<Vec<i32>>(),
        );

        self.pos = passed_pos;

        let mut comparisons = [self.main_align, self.cross_align];

        if self.direction == Direction::Column {
            comparisons.swap(0, 1);
        }

        if comparisons[0] != Alignment::Start {
            let self_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
            let children_width = self
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().0);
            let children_width = match self.direction {
                Direction::Row => children_width.sum(),
                Direction::Column => children_width.max().unwrap(),
            };
            let total_gap = self.gap * (self.children.len() as i32 - 1);
            let remaining_space = self_width - children_width - total_gap;

            if comparisons[0] == Alignment::Center {
                if self.direction == Direction::Column {
                    // If column and cross align center, each child is centered
                    for (idx, child) in self.children.iter().enumerate() {
                        let child_width = child.borrow().get_draw_dim().0;
                        cross_paddings[idx] = self.padding.0 + (self_width - child_width) / 2;
                    }
                } else {
                    padding_left =  self.padding.0 + remaining_space / 2;
                }
            }

            if comparisons[0] == Alignment::End {
                padding_left = self.padding.0 + remaining_space;
            }
        }

        if comparisons[1] != Alignment::Start {
            let self_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
            let children_height = self
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().1);
            let children_height = match self.direction {
                Direction::Row => children_height.max().unwrap(),
                Direction::Column => children_height.sum(),
            };
            let total_gap = self.gap * (self.children.len() as i32 - 1);
            let remaining_space = self_height - children_height - total_gap;
            if comparisons[1] == Alignment::Center {
                padding_top = self.padding.1 + remaining_space / 2;
            }
            if comparisons[1] == Alignment::End {
                padding_top = self.padding.1 + remaining_space;
            }
        }

        let mut next_pos = self.pos;
        if self.direction == Direction::Column && self.cross_align == Alignment::Center {
            next_pos.0 = self.pos.0 + cross_paddings[0];
        } else {
            next_pos.0 += padding_left;
        }
        next_pos.1 += padding_top;

        for (idx, child) in self.children.iter().enumerate() {
            child.borrow_mut().pass_2(next_pos);
            let (child_width, child_height) = child.borrow().get_draw_dim();
            if idx < self.children.len() - 1 {
                match self.direction {
                    Direction::Row => next_pos.0 += child_width + self.gap,
                    Direction::Column => {
                        if self.cross_align == Alignment::Center {
                            next_pos.0 = self.pos.0 + cross_paddings[idx + 1];
                        }
                        next_pos.1 += child_height + self.gap
                    }
                }
            }
        }
    }
    fn get_flex(&self) -> f32 {
        self.flex
    }
    fn debug_dims(&self, depth: usize) {
        tabbed_print(
            &format!(
                "<layout width={} height={} x={} y={} padding=({},{},{},{}) gap={} dir={:?} main_align={:?} cross_align={:?} name='{}' flex={}>",
                self.draw_dim.0,
                self.draw_dim.1,
                self.pos.0,
                self.pos.1,
                self.padding.0,
                self.padding.1,
                self.padding.2,
                self.padding.3,
                self.gap,
                self.direction,
                self.main_align,
                self.cross_align,
                self.dbg_name,
                self.flex
            ),
            depth,
        );
        for (_i, child) in self.children.iter().enumerate() {
            child.borrow().debug_dims(depth + 1);
        }
        tabbed_print("</layout>", depth);
    }
}
