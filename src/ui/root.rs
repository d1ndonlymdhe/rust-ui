use raylib::{
    color::Color, ffi::KeyboardKey, prelude::{RaylibDraw, RaylibDrawHandle}
};

use crate::ui::common::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

pub struct Root {
    pub child: Rc<RefCell<dyn Base>>,
    pub draw_dim: (i32, i32),
    pub pos: (i32, i32),
    pub focused_id: Option<String>,
    pub scroll_map: HashMap<String, i32>,
}

impl Root {
    pub fn draw(&mut self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.clear_background(Color::BLACK);
        {
            let mut child_mut = self.child.borrow_mut();
            child_mut.set_pos(self.pos);
            child_mut.set_raw_dim(self.draw_dim);
        }
        let mut abs_draw = vec![];
        let child = self.child.borrow();
        abs_draw = child.draw(draw_handle);
        loop {
            let mut new_abs_draws = vec![];
            for draw_instruction in abs_draw.iter() {
                let AbsoluteDraw {
                    component_id,
                    container_y,
                    y_offset,
                    ..
                } = draw_instruction;

                let instructed_height = draw_instruction.container_height;

                let child = self.get_by_id(component_id);
                if let Some(child) = child {
                    let child = child.borrow();
                    let child_pos = child.get_position();
                    match child_pos {
                        Position::Auto => {
                            panic!("No auto children should exist here")
                        }
                        Position::GlobalAbsolute(_, _) => {
                            let more_abs_draw = child.draw(
                                draw_handle,
                                
                            );
                            new_abs_draws.extend(more_abs_draw);
                        }
                        Position::LocalAbsolute(_, _) => {
                            let more_abs_draw = child.draw(
                                draw_handle,
                                
                            );
                            new_abs_draws.extend(more_abs_draw);
                        }
                    }
                }
            }
            if new_abs_draws.is_empty() {
                break;
            } else {
                abs_draw = new_abs_draws;
            }
        }
    }

    pub fn handle_key_event(&self, key_event: KeyEvent) -> bool {
        if key_event.ctrl_down && key_event.key.is_some_and(|v|{v==KeyboardKey::KEY_D}){
            println!("DEBUG DIMS");
            self.debug_dims(0);
        }
        if let Some(focused_id) = &self.focused_id {
            if let Some(focused_child) = self.get_by_id(focused_id) {
                let focused_child = focused_child.borrow();
                focused_child.execute_on_key(key_event);
                return true;
            }
        }
        false
    }
    pub fn get_mouse_event_handlers(&mut self, mouse_event: MouseEvent) -> bool {
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
            return true;
        }
        false
    }

    pub fn get_scroll_event_handler(&mut self, scroll_event: ScrollEvent) -> bool {
        if scroll_event.delta == 0 {
            return false;
        }
        let child = self.child.clone();
        if let Some(handler_id) = child.borrow().get_scroll_event_handler(scroll_event) {
            let scroll_map = &mut self.scroll_map;
            let entry = scroll_map.entry(handler_id);
            let scroll_offset = entry.or_insert(0);
            *scroll_offset -= scroll_event.delta * 15;
            return true;
            // println!("New scroll offset: {}", scroll_offset);
            // let child = self.get_by_id(&handler_id);
            // if let Some(child) = child {

            //     println!("Found scroll handler: {}", handler_id);
            //     // let mut child = child.borrow_mut();
            //     // child.execute_on_scroll(scroll_event);
            // }
        }
        false
    }

    pub fn pass_1(&mut self) {
        let mut mut_child = self.child.borrow_mut();
        mut_child.set_raw_dim(self.draw_dim);
        mut_child.pass_1(self.draw_dim, 0);
    }
    pub fn pass_2(&mut self) {
        self.child.borrow_mut().pass_2(self.pos);
    }
    pub fn pass_overflow(&mut self){
        self.child.borrow_mut().pass_overflow((self.draw_dim), self.pos, &mut self.scroll_map, 0);
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
    pub fn set_children(&mut self, children: Vec<Rc<RefCell<dyn Base>>>) {
        if children.len() != 1 {
            panic!("Root can only have one child");
        }
        self.child = children.into_iter().next().unwrap();
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
}

impl Root {
    pub fn new(child: Rc<RefCell<dyn Base>>, dim: (i32, i32)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            child,
            draw_dim: dim,
            pos: (0, 0),
            focused_id: None,
            scroll_map: HashMap::new(),
        }))
    }
}
