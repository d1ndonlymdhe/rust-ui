use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    FILL,
    FIT,
    FIXED(i32),
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

pub trait Base {
    fn set_pos(&mut self, pos: (i32, i32));
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
    fn set_dim(&mut self, parent_draw_dim: (i32, i32));
    fn get_draw_dim(&self) -> (i32, i32);
    fn pass_1(&mut self, parent_draw_dim: (i32, i32));
    fn pass_2(&mut self, parent_pos: (i32, i32));
    fn get_flex(&self) -> f32;
    fn debug_dims(&self, depth: usize);
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
    };

    (draw_width, draw_height)
}

pub fn tabbed_print(text: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{}", indent, text);
}