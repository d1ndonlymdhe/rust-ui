use std::{cell::RefCell, ffi::CString, rc::Rc};

use raylib::prelude::*;

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Rust UI Example")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let div1 = Div::new(
            Rc::new(RefCell::new(Text::new("Hello, world!", 32))),
            (Length::FIT, Length::FIT),
            Color::DARKGRAY,
        );
        let div2 = Div::new(
            Rc::new(RefCell::new(Text::new("This is a test.", 32))),
            (Length::FIT, Length::FIT),
            Color::LIGHTGRAY,
        );

        let row = Row::new(vec![div1, div2], (Length::FILL, Length::FIT), Color::RED);
        let root = Root::new(row, (800, 600));
        root.draw(&mut d);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Length {
    FILL,
    FIT,
    FIXED(i32),
}

trait Base {
    // fn get_pos(&self) -> (i32, i32);
    fn set_pos(&mut self, pos: (i32, i32)) -> ();
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> ();
    fn set_dim(&mut self, parent_draw_dim: (i32, i32)) -> ();
    fn get_draw_dim(&self) -> (i32, i32);
}

struct Root {
    child: Rc<RefCell<dyn Base>>,
    dim: (Length, Length),
    draw_dim: (i32, i32),
    pos: (i32, i32),
}

impl Base for Root {
    fn set_pos(&mut self, _pos: (i32, i32)) {
        panic!("Root cannot have parent");
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> () {
        let child = self.child.clone();
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
}

impl Root {
    fn new(child: Rc<RefCell<dyn Base>>, dim: (i32, i32)) -> Self {
        Self {
            child,
            dim: (Length::FIXED(dim.0), Length::FIXED(dim.1)),
            draw_dim: dim,
            pos: (0, 0),
        }
    }

    fn set_window_dim(&mut self, window_dim: (i32, i32)) -> () {
        let (draw_width, draw_height) =
            get_draw_dim(self.dim, window_dim, &vec![self.child.clone()]);
        self.draw_dim = (draw_width, draw_height);
    }
}

struct Row {
    children: Vec<Rc<RefCell<dyn Base>>>,
    dim: (Length, Length),
    draw_dim: (i32, i32),
    pos: (i32, i32),
    bg_color: Color,
}

impl Base for Row {
    fn set_pos(&mut self, pos: (i32, i32)) -> () {
        self.pos = pos;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> () {
        let mut next_pos = self.pos;
        draw_handle.draw_rectangle(
            self.pos.0,
            self.pos.1,
            self.draw_dim.0,
            self.draw_dim.1,
            self.bg_color,
        );
        for child in self.children.iter() {
            let child = child.clone();

            child.borrow_mut().set_dim((
                self.draw_dim.0 / self.children.len() as i32,
                self.draw_dim.1,
            ));
            child.borrow_mut().set_pos(next_pos);

            child.borrow_mut().draw(draw_handle);

            let (child_width, _child_height) = child.borrow().get_draw_dim();
            next_pos.0 += child_width;
        }
    }

    fn set_dim(&mut self, parent_dim: (i32, i32)) -> () {
        let (draw_width, draw_height) = get_draw_dim(self.dim, parent_dim, &self.children);
        self.draw_dim = (draw_width, draw_height);
    }

    fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
}

impl Row {
    fn new(children: Vec<Rc<RefCell<dyn Base>>>, dim: (Length, Length), bg_color: Color) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            children,
            dim,
            draw_dim: (0, 0),
            pos: (0, 0),
            bg_color
        }))
    }
}

fn get_draw_dim(
    dim: (Length, Length),
    parent_dim: (i32, i32),
    children: &Vec<Rc<RefCell<dyn Base>>>,
) -> (i32, i32) {
    let (width, height) = dim;

    let draw_width = match width {
        Length::FILL => parent_dim.0,
        Length::FIT => children
            .iter()
            .map(|child| child.borrow().get_draw_dim().0)
            .sum(),
        Length::FIXED(l) => l,
    };

    let draw_height = match height {
        Length::FILL => parent_dim.1,
        Length::FIT => children
            .iter()
            .map(|child| child.borrow().get_draw_dim().1)
            .sum(),
        Length::FIXED(l) => l,
    };

    (draw_width, draw_height)
}

struct Div {
    child: Rc<RefCell<dyn Base>>,
    dim: (Length, Length),
    draw_dim: (i32, i32),
    pos: (i32, i32),
    bg_color: Color,
}

impl Base for Div {
    fn set_pos(&mut self, pos: (i32, i32)) -> () {
        self.pos = pos;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> () {
        draw_handle.draw_rectangle(
            self.pos.0,
            self.pos.1,
            self.draw_dim.0,
            self.draw_dim.1,
            self.bg_color,
        );
        self.child.borrow_mut().set_pos(self.pos);
        self.child.borrow().draw(draw_handle);
    }

    fn set_dim(&mut self, parent_draw_dim: (i32, i32)) -> () {
        let (draw_width, draw_height) =
            get_draw_dim(self.dim, parent_draw_dim, &vec![self.child.clone()]);
        self.draw_dim = (draw_width, draw_height);
    }

    fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
}

impl Div {
    fn new(
        child: Rc<RefCell<dyn Base>>,
        dim: (Length, Length),
        bg_color: Color,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            child,
            dim,
            draw_dim: (0, 0),
            pos: (0, 0),
            bg_color,
        }))
    }
}

struct Text {
    content: String,
    font_size: i32,
    pos: (i32, i32),
}

impl Base for Text {
    fn set_pos(&mut self, pos: (i32, i32)) -> () {
        self.pos = pos;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> () {
        draw_handle.draw_text(
            &self.content,
            self.pos.0,
            self.pos.1,
            self.font_size,
            Color::WHITE,
        );
    }

    fn set_dim(&mut self, _parent_draw_dim: (i32, i32)) -> () {
        ()
    }

    fn get_draw_dim(&self) -> (i32, i32) {
        let width;
        unsafe {
            let c_text = CString::new(self.content.as_str()).unwrap();
            width = raylib::ffi::MeasureText(c_text.as_ptr(), self.font_size);
        }
        (width as i32, self.font_size as i32)
    }
}

impl Text {
    fn new(content: &str, font_size: i32) -> Self {
        Self {
            content: content.to_string(),
            font_size,
            pos: (0, 0),
        }
    }
}
