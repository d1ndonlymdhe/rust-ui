use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::ui::common::*;
use std::{cell::RefCell, rc::Rc};

pub struct Root {
    pub child: Rc<RefCell<dyn Base>>,
    pub draw_dim: (i32, i32),
    pub pos: (i32, i32),
}

impl Base for Root {
    fn set_pos(&mut self, _pos: (i32, i32)) {
        panic!("Root cannot have parent");
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let child = self.child.clone();
        draw_handle.clear_background(Color::BLACK);
        child.clone().borrow_mut().set_pos(self.pos);
        child.clone().borrow_mut().set_dim(self.draw_dim);
        child.clone().borrow_mut().draw(draw_handle);
    }
    fn set_dim(&mut self, _parent_dim: (i32, i32)) {
        panic!("Root cannot have parent");
    }
    fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
    fn pass_1(&mut self, _parent_draw_dim: (i32, i32)) {
        self.child.borrow_mut().set_dim(self.draw_dim);
        self.child.borrow_mut().pass_1(self.draw_dim);
    }
    fn pass_2(&mut self, _parent_pos: (i32, i32)) {
        self.child.borrow_mut().pass_2(self.pos);
    }
    fn debug_dims(&self, depth: usize) {
        tabbed_print(
            &format!(
                "<root width={} height={} x={} y={} >",
                self.draw_dim.0, self.draw_dim.1, self.pos.0, self.pos.1
            ),
            depth,
        );
        self.child.borrow().debug_dims(depth + 1);
        tabbed_print("</root>", depth);
    }
    fn get_flex(&self) -> f32 {
        1.0
    }
}

impl Root {
    pub fn new(child: Rc<RefCell<dyn Base>>, dim: (i32, i32)) -> Self {
        Self {
            child,
            draw_dim: dim,
            pos: (0, 0),
        }
    }
}
