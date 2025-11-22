use crate::ui::common::*;
use raylib::prelude::*;
use std::{cell::RefCell, collections::HashMap, ffi::CString, rc::Rc};

pub struct RawText {
    pub content: String,
    pub font_size: i32,
    pub pos: (i32, i32),
    pub dbg_name: ID,
    pub padding: (i32, i32, i32, i32), // top, right, bottom, left
    pub color: Color,
}

impl Base for RawText {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
    fn get_draw_pos(&self) -> (i32, i32) {
        self.pos
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle,container_y:i32,container_height: i32, _scroll_map: &HashMap<String, i32>,y_offset:i32) {
        // let max_scroll = (self.font_size - container_height).max(0);
        // let scroll_top = scroll_map
        //     .get(&self.get_id())
        //     .cloned()
        //     .unwrap_or(0)
        //     //TODO find correct formula (padding)
        //     .clamp(0, max_scroll);
        //  let (_, visible_height) = get_drawable_y_and_h(
        //      y_offset,
        //      container_y,
        //      container_height,
        //      self.pos.1,
        //      self.get_draw_dim().1,
        //  );
        //  if visible_height <= self.font_size {
        //     return;
        //  }
        draw_handle.draw_text(
            &self.content,
            self.pos.0,
            self.pos.1 - y_offset,
            self.font_size,
            self.color,
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
        (
            width + self.padding.0 + self.padding.2,
            self.font_size + self.padding.1 + self.padding.3,
        )
    }
    fn pass_1(&mut self, parent_draw_dim: (i32, i32), id: usize) -> usize {
        self.set_dim(parent_draw_dim);
        let ret_id = id + 1;
        if let ID::Auto(_) = &self.dbg_name {
            self.dbg_name = ID::Auto(ret_id.to_string());
        }
        ret_id
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
    fn add_child(&mut self, child: Rc<RefCell<dyn Base>>) {
        ()
    }

    fn get_mouse_event_handlers(&self, _mouse_event: MouseEvent) -> Vec<String> {
        Vec::new()
    }
    fn set_children(&mut self, _children: Vec<Rc<RefCell<dyn Base>>>) {
        panic!("RawText cannot have children");
    }
    fn on_click(&mut self, _f: Box<dyn FnMut(MouseEvent) -> bool>) {
        panic!("RawText cannot have on_click");
    }
    fn get_id(&self) -> String {
        match &self.dbg_name {
            ID::Auto(name) => name.clone(),
            ID::Manual(name) => name.clone(),
        }
    }
    fn get_by_id(&self, _id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        None
    }

    fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>> {
        Rc::new(RefCell::new(|_mouse_event| true))
    }

    fn get_key_event_handlers(&self, key_event: KeyEvent) -> Vec<String> {
        vec![]
    }

    fn get_on_key(&self) -> Rc<RefCell<dyn FnMut(KeyEvent) -> bool>> {
        Rc::new(RefCell::new(|_key_event| true))
    }
    fn get_overflow(&self) -> (bool, bool) {
        (false, false)
    }
    fn get_scroll_event_handler(&self, _scroll_event: ScrollEvent) -> Option<String> {
        None
    }
}

impl RawText {
    pub fn new(content: &str, font_size: i32, padding: (i32, i32, i32, i32),color: Color) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content: content.to_string(),
            font_size,
            pos: (0, 0),
            padding,
            dbg_name: ID::Auto(generate_id()),
            color,
        }))
    }
}
