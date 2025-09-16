use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

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
        draw_handle.clear_background(Color::BLACK);
        {
            let mut child_mut = self.child.borrow_mut();
            child_mut.set_pos(self.pos);
            child_mut.set_dim(self.draw_dim);
        }
        let child = self.child.borrow();
        child.draw(draw_handle);
    }
    fn handle_mouse_event(&self, mouse_event: MouseEvent) -> bool {
        let child = self.child.clone();
        child.borrow().handle_mouse_event(mouse_event)
    }
    fn on_click(&mut self, _f: Box<dyn FnMut(MouseEvent) -> bool>) {
        ()
    }
    fn set_dim(&mut self, _parent_dim: (i32, i32)) {
        panic!("Root cannot have parent");
    }
    fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
    fn pass_1(&mut self, _parent_draw_dim: (i32, i32)) {
        let mut mut_child = self.child.borrow_mut();
        mut_child.set_dim(self.draw_dim);
        mut_child.pass_1(self.draw_dim);
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
    fn set_children(&mut self, children: Vec<Rc<RefCell<dyn Base>>>) {
        if children.len() != 1 {
            panic!("Root can only have one child");
        }
        self.child = children.into_iter().next().unwrap();
    }
    fn get_id(&self) -> String {
        "root".to_string()
    }
    fn get_by_id(&self, id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        let child = self.child.clone();

        let is_target = {
            match child.try_borrow_mut() {
                Ok(borrowed_child) => {
                    println!("Successfully borrowed child for ID check");
                    borrowed_child.get_id() == id},
                Err(_) => {
                    println!("Failed to borrow child for ID check");
                    false
                }
            };

            let borrowed_child = child.borrow();
            borrowed_child.get_id() == id
        };
        if is_target {
            // Now we can safely try to borrow mutably
            match self.child.try_borrow_mut() {
                Ok(_) => Some(self.child.clone()),
                Err(_) => {
                    println!("Failed to borrow div");
                    None
                }
            }
        } else {
            child.borrow().get_by_id(id)
        }
    }
}

impl Root {
    pub fn new(child: Rc<RefCell<dyn Base>>, dim: (i32, i32)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            child,
            draw_dim: dim,
            pos: (0, 0),
        }))
    }
}
