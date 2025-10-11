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

    fn get_key_event_handlers(&self, key_event: KeyEvent) -> Vec<String>;
    fn execute_on_key(&self, key_event: KeyEvent) -> bool {
        let f = self.get_on_key();
        let mut f = f.borrow_mut();
        f(key_event)
    }
    fn get_on_key(&self) -> Rc<RefCell<dyn FnMut(KeyEvent) -> bool>>;
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
    fn is_focusable(&self) -> bool {
        false
    }
    fn set_children_func(
        &mut self,
        f: Option<Rc<RefCell<dyn Fn() -> Vec<Rc<RefCell<dyn Base>>>>>>,
    );
}

#[derive(Clone, Copy, Debug)]
pub struct MouseEvent {
    pub pos: (i32, i32),
    pub left_button_down: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    pub key: Option<KeyboardKey>,
    pub shift_down: bool,
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

pub fn keyboard_key_to_char(key: KeyboardKey) -> Option<char> {
    match key {
        KeyboardKey::KEY_A => Some('a'),
        KeyboardKey::KEY_B => Some('b'),
        KeyboardKey::KEY_C => Some('c'),
        KeyboardKey::KEY_D => Some('d'),
        KeyboardKey::KEY_E => Some('e'),
        KeyboardKey::KEY_F => Some('f'),
        KeyboardKey::KEY_G => Some('g'),
        KeyboardKey::KEY_H => Some('h'),
        KeyboardKey::KEY_I => Some('i'),
        KeyboardKey::KEY_J => Some('j'),
        KeyboardKey::KEY_K => Some('k'),
        KeyboardKey::KEY_L => Some('l'),
        KeyboardKey::KEY_M => Some('m'),
        KeyboardKey::KEY_N => Some('n'),
        KeyboardKey::KEY_O => Some('o'),
        KeyboardKey::KEY_P => Some('p'),
        KeyboardKey::KEY_Q => Some('q'),
        KeyboardKey::KEY_R => Some('r'),
        KeyboardKey::KEY_S => Some('s'),
        KeyboardKey::KEY_T => Some('t'),
        KeyboardKey::KEY_U => Some('u'),
        KeyboardKey::KEY_V => Some('v'),
        KeyboardKey::KEY_W => Some('w'),
        KeyboardKey::KEY_X => Some('x'),
        KeyboardKey::KEY_Y => Some('y'),
        KeyboardKey::KEY_Z => Some('z'),

        KeyboardKey::KEY_SPACE => Some(' '),
        KeyboardKey::KEY_ENTER => Some('\n'),

        KeyboardKey::KEY_COMMA => Some(','),
        KeyboardKey::KEY_PERIOD => Some('.'),
        KeyboardKey::KEY_APOSTROPHE => Some('\''),
        KeyboardKey::KEY_SEMICOLON => Some(';'),
        KeyboardKey::KEY_SLASH => Some('/'),
        KeyboardKey::KEY_BACKSLASH => Some('\\'),
        KeyboardKey::KEY_LEFT_BRACKET => Some('['),
        KeyboardKey::KEY_RIGHT_BRACKET => Some(']'),
        KeyboardKey::KEY_MINUS => Some('-'),
        KeyboardKey::KEY_EQUAL => Some('='),
        KeyboardKey::KEY_GRAVE => Some('`'),

        KeyboardKey::KEY_ZERO => Some('0'),
        KeyboardKey::KEY_ONE => Some('1'),
        KeyboardKey::KEY_TWO => Some('2'),
        KeyboardKey::KEY_THREE => Some('3'),
        KeyboardKey::KEY_FOUR => Some('4'),
        KeyboardKey::KEY_FIVE => Some('5'),
        KeyboardKey::KEY_SIX => Some('6'),
        KeyboardKey::KEY_SEVEN => Some('7'),
        KeyboardKey::KEY_EIGHT => Some('8'),
        KeyboardKey::KEY_NINE => Some('9'),
        _ => None,
    }
}


pub fn def_key_handler(key_event: KeyEvent,text: &mut String) -> bool {
    if let Some(key) = key_event.key {
        if key == KeyboardKey::KEY_BACKSPACE {
            text.pop();
            return true;
        }
        if let Some(mut c) = keyboard_key_to_char(key) {
            if key_event.shift_down {
                c = shift_character(c);
            }
            text.push(c);
            return true;
        }
    }
    false
}

pub fn shift_character(c: char) -> char {
    match c {
        'a'..='z' => ((c as u8) - 32) as char,
        'A'..='Z' => c,
        '0' => ')',
        '1' => '!',
        '2' => '@',
        '3' => '#',
        '4' => '$',
        '5' => '%',
        '6' => '^',
        '7' => '&',
        '8' => '*',
        '9' => '(',
        '-' => '_',
        '=' => '+',
        '[' => '{',
        ']' => '}',
        '\\' => '|',
        ';' => ':',
        '\'' => '"',
        ',' => '<',
        '.' => '>',
        '/' => '?',
        '`' => '~',
        _ => c,
    }
}

pub type Component = Rc<RefCell<dyn Base>>;