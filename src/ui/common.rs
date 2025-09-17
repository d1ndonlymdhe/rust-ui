use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    FILL,
    FIT,
    FIXED(i32),
    PERCENT(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}
use uuid::Uuid;
pub trait Base {
    fn set_pos(&mut self, pos: (i32, i32));
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
    fn get_mouse_event_handlers(&self, mouse_event: MouseEvent) -> Vec<String>;
    fn execute_on_click(&self, mouse_event: MouseEvent) -> bool {
        let f = self.get_on_click();
        let mut f = f.borrow_mut();
        f(mouse_event)
    }
    fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>>;
    fn set_dim(&mut self, parent_draw_dim: (i32, i32));
    fn get_draw_dim(&self) -> (i32, i32);
    fn get_draw_pos(&self) -> (i32, i32);
    fn pass_1(&mut self, parent_draw_dim: (i32, i32));
    fn pass_2(&mut self, parent_pos: (i32, i32));
    fn get_flex(&self) -> f32;
    fn debug_dims(&self, depth: usize);
    fn set_children(&mut self, children: Vec<Rc<RefCell<dyn Base>>>);
    fn get_children(&self) -> Vec<Rc<RefCell<dyn Base>>> {
        Vec::new()
    }
    fn add_child(&mut self, child: Rc<RefCell<dyn Base>>) {
        let mut children = self.get_children();
        children.push(child);
        self.set_children(children);
    }
    fn on_click(&mut self, f: Box<dyn FnMut(MouseEvent) -> bool>);
    fn get_id(&self) -> String;
    fn get_by_id(&self, id: &str) -> Option<Rc<RefCell<dyn Base>>>;
}

#[derive(Clone, Copy, Debug)]
pub struct MouseEvent {
    pub pos: (i32, i32),
    pub left_button_down: bool,
}

pub fn get_draw_dim(
    dim: (Length, Length),
    parent_dim: (i32, i32),
    children: &Vec<Rc<RefCell<dyn Base>>>,
    
    direction: &Direction,
) -> (i32, i32) {
    let (width, height) = dim;

    let draw_width = match width {
        Length::FILL => parent_dim.0,
        Length::FIT => {
            let iter = children.iter().map(|child| child.borrow().get_draw_dim().0);
            match direction {
                Direction::Row => iter.sum(),
                Direction::Column => iter.max().unwrap(),
            }
        }
        Length::FIXED(l) => l,
        Length::PERCENT(p) => (parent_dim.0 * p) / 100,
    };

    let draw_height = match height {
        Length::FILL => parent_dim.1,
        Length::FIT => {
            let iter = children.iter().map(|child| child.borrow().get_draw_dim().1);
            match direction {
                Direction::Row => iter.max().unwrap(),
                Direction::Column => iter.sum(),
            }
        }
        Length::FIXED(l) => l,
        Length::PERCENT(p) => (parent_dim.1 * p) / 100,
    };

    (draw_width, draw_height)
}

pub fn tabbed_print(text: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{}", indent, text);
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}