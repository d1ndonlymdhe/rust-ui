use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::ui::common::*;
use std::{cell::RefCell, rc::Rc, vec};

pub struct Root {
    pub child: Rc<RefCell<dyn Base>>,
    pub draw_dim: (i32, i32),
    pub pos: (i32, i32),
    pub focused_id: Option<String>,
}

impl Root {
    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.clear_background(Color::BLACK);
        {
            let mut child_mut = self.child.borrow_mut();
            child_mut.set_pos(self.pos);
            child_mut.set_dim(self.draw_dim);
        }
        let child = self.child.borrow();
        child.draw(draw_handle);
    }

    pub fn handle_key_event(&self, key_event: KeyEvent) {
        // let child = self.child.clone();
        // let hit_children = child.borrow().get_key_event_handlers(key_event);
        // for child_id in hit_children.iter() {
        //     let child = self.get_by_id(&child_id);
        //     if let Some(child) = child {
        //         let child = child.borrow();
        //         let propagate = child.execute_on_key(key_event);
        //         if !propagate {
        //             break;
        //         }
        //     }
        // }
        // vec![]
        // println!("Focused ID: {:?}", self.focused_id);
        if let Some(focused_id) = &self.focused_id {
            if let Some(focused_child) = self.get_by_id(focused_id) {
                let focused_child = focused_child.borrow();
                focused_child.execute_on_key(key_event);
            }
        }
    }
    pub fn get_mouse_event_handlers(&mut self, mouse_event: MouseEvent) {
        let child = self.child.clone();
        let hit_children = child.borrow().get_mouse_event_handlers(mouse_event);

        let mut focused_id = None;

        for child_id in hit_children.iter() {
            let child = self.get_by_id(&child_id);
            if let Some(child) = child {
                let child = child.borrow();
                let propagate = child.execute_on_click(mouse_event);
                if child.is_focusable() && focused_id.is_none() {
                    focused_id = Some(child_id.clone());
                }
                if !propagate {
                    break;
                }
            }
        }
        if mouse_event.left_button_down {
            self.focused_id = focused_id;
        }
    }
    pub fn on_click(&mut self, _f: Box<dyn FnMut(MouseEvent) -> bool>) {
        ()
    }
    pub fn set_dim(&mut self, _parent_dim: (i32, i32)) {
        panic!("Root cannot have parent");
    }
    pub fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
    pub fn pass_1(&mut self, _parent_draw_dim: (i32, i32)) {
        let mut mut_child = self.child.borrow_mut();
        mut_child.set_dim(self.draw_dim);
        mut_child.pass_1(self.draw_dim);
    }
    pub fn pass_2(&mut self, _parent_pos: (i32, i32)) {
        self.child.borrow_mut().pass_2(self.pos);
    }
    pub fn debug_dims(&self, depth: usize) {
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
    pub fn get_flex(&self) -> f32 {
        1.0
    }
    pub fn set_children(&mut self, children: Vec<Rc<RefCell<dyn Base>>>) {
        if children.len() != 1 {
            panic!("Root can only have one child");
        }
        self.child = children.into_iter().next().unwrap();
    }
    pub fn get_id(&self) -> String {
        "root".to_string()
    }
    pub fn get_by_id(&self, id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        let child = self.child.clone();

        let is_target = {
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

    pub fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>> {
        Rc::new(RefCell::new(|_mouse_event| true))
    }
}

impl Root {
    pub fn new(child: Rc<RefCell<dyn Base>>, dim: (i32, i32)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            child,
            draw_dim: dim,
            pos: (0, 0),
            focused_id: None,
        }))
    }
}
