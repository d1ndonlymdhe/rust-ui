use crate::ui::common::*;
use raylib::prelude::*;
use std::{cell::RefCell, ffi::CString, rc::Rc};

pub struct RawText {
    pub content: String,
    pub font_size: i32,
    pub pos: (i32, i32),
}

impl Base for RawText {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_text(
            &self.content,
            self.pos.0,
            self.pos.1,
            self.font_size,
            Color::WHITE,
        );
    }
    fn set_dim(&mut self, _parent_draw_dim: (i32, i32)) {
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
    fn pass_1(&mut self, parent_draw_dim: (i32, i32)) {
        self.set_dim(parent_draw_dim);
    }
    fn pass_2(&mut self, parent_pos: (i32, i32)) {
        self.pos = (parent_pos.0, parent_pos.1);
    }
    fn debug_dims(&self, depth: usize) {
        tabbed_print(
            &format!(
                "<text width={} height={} x={} y={}>",
                self.get_draw_dim().0,
                self.get_draw_dim().1,
                self.pos.0,
                self.pos.1
            ),
            depth,
        );
        tabbed_print(&self.content, depth + 1);
        tabbed_print("</text>", depth);
    }
    fn get_flex(&self) -> f32 {
        1.0
    }

    fn handle_mouse_event(&self, _mouse_event: MouseEvent) -> bool {
        true
    }
    fn set_children(&mut self, _children: Vec<Rc<RefCell<dyn Base>>>) {
        panic!("RawText cannot have children");
    }
    fn on_click(&mut self, _f: Box<dyn FnMut(MouseEvent) -> bool>) {
        panic!("RawText cannot have on_click");
    }
    fn get_id(&self) -> String {
        "".to_string()
    }
    fn get_by_id(&self, _id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        None
    }
}

impl RawText {
    pub fn new(content: &str, font_size: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content: content.to_string(),
            font_size,
            pos: (0, 0),
        }))
    }
}
