use std::{cell::RefCell, collections::HashMap, ffi::CString, rc::Rc};

use raylib::{
    color::Color,
    ffi::KeyboardKey,
    prelude::RaylibDrawHandle,
};

use crate::ui::{
    common::{
        AbsoluteDraw, Alignment, Base, KeyEvent, Length, MouseEvent, keyboard_key_to_char, shift_character, tabbed_print
    },
    layout::{Layout, LayoutProps},
    raw_text::RawText,
};

pub struct TextInputProps {
    pub layout: LayoutProps,
    pub content: Rc<RefCell<String>>,
    pub font_size: i32,
    pub wrap: bool,
    pub text_color: Color,
    pub def_on_key: Rc<RefCell<dyn FnMut(KeyEvent) -> bool>>,
}

impl TextInputProps {
    pub fn new() -> Self {
        let layout = Layout::get_col_builder()
            .dim((Length::FIT, Length::FIT))
            .bg_color(Color::WHITE)
            .main_align(Alignment::Start)
            .cross_align(Alignment::Start)
            .padding((0, 0, 0, 0))
            .gap(0)
            .flex(1.0)
            .overflow_x(false)
            .overflow_y(true);
        
        let text_content = Rc::new(RefCell::new(String::new()));
        let closure_text_content = text_content.clone();
        let def_on_key = Rc::new(RefCell::new(move |key_event: KeyEvent| {
            if let Some(key) = key_event.key {
                match key {
                    KeyboardKey::KEY_BACKSPACE => {
                        let mut content = closure_text_content.borrow_mut();
                        content.pop();
                    }
                    _ => {
                        if let Some(c) = keyboard_key_to_char(key) {
                            let mut c = c;
                            let mut content = closure_text_content.borrow_mut();
                            if key_event.shift_down {
                                c = shift_character(c);
                            }
                            content.push(c);
                        }
                    }
                }
            }
            true
        }));

        Self {
            layout,
            content: text_content,
            font_size: 24,
            wrap: true,
            text_color: Color::BLACK,
            def_on_key,
        }
    }

    pub fn content(self, content: &str) -> Self {
        self.content.replace(content.into());
        self
    }

    pub fn font_size(mut self, size: i32) -> Self {
        self.font_size = size;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn dim(mut self, dim: (Length, Length)) -> Self {
        let layout = self.layout.dim(dim);
        self.layout = layout;
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        let layout = self.layout.bg_color(color);
        self.layout = layout;
        self
    }

    pub fn main_align(mut self, align: Alignment) -> Self {
        let layout = self.layout.main_align(align);
        self.layout = layout;
        self
    }

    pub fn cross_align(mut self, align: Alignment) -> Self {
        let layout = self.layout.cross_align(align);
        self.layout = layout;
        self
    }

    pub fn padding(mut self, padding: (i32, i32, i32, i32)) -> Self {
        let layout = self.layout.padding(padding);
        self.layout = layout;
        self
    }

    pub fn gap(mut self, gap: i32) -> Self {
        let layout = self.layout.gap(gap);
        self.layout = layout;
        self
    }

    pub fn dbg_name(mut self, name: &str) -> Self {
        let layout = self.layout.dbg_name(name);
        self.layout = layout;
        self
    }

    pub fn flex(mut self, flex: f32) -> Self {
        let layout = self.layout.flex(flex);
        self.layout = layout;
        self
    }

    pub fn on_click(mut self, f: Box<dyn FnMut(MouseEvent) -> bool>) -> Self {
        let layout = self.layout.on_click(f);
        self.layout = layout;
        self
    }

    pub fn on_key(mut self, f: Box<dyn FnMut(KeyEvent) -> bool>) -> Self {
        self.def_on_key = Rc::new(RefCell::new(f));
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn overflow_x(mut self, overflow: bool) -> Self {
        let layout = self.layout.overflow_x(overflow);
        self.layout = layout;
        self
    }

    pub fn overflow_y(mut self, overflow: bool) -> Self {
        let layout = self.layout.overflow_y(overflow);
        self.layout = layout;
        self
    }

    pub fn build(self) -> Rc<RefCell<TextInput>> {
        let layout = self.layout;
        Rc::new(RefCell::new(TextInput {
            layout: layout.get_layout(),
            content: self.content,
            font_size: self.font_size,
            wrap: self.wrap,
            text_color: self.text_color,
            def_on_key: self.def_on_key,
        }))
    }
}

pub struct TextInput {
    layout: Layout,
    content: Rc<RefCell<String>>,
    font_size: i32,
    wrap: bool,
    text_color: Color,
    def_on_key: Rc<RefCell<dyn FnMut(KeyEvent) -> bool>>,
}

impl TextInput {
    pub fn get_builder() -> TextInputProps {
        TextInputProps::new()
    }

    pub fn get_content(&self) -> String {
        self.content.borrow().clone()
    }
}

fn get_text_rows(content: &str, max_width: i32, font_size: i32) -> Vec<String> {
    let mut rows = vec![];
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut current_row = String::new();
    for word in words {
        let test_row = if current_row.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_row, word)
        };
        let text_width;
        unsafe {
            let c_text = CString::new(test_row.as_str()).unwrap();
            text_width = raylib::ffi::MeasureText(c_text.as_ptr(), font_size);
        }
        if text_width <= max_width {
            current_row = test_row;
        } else {
            if !current_row.is_empty() {
                rows.push(current_row);
            }
            current_row = word.to_string();
        }
    }
    if !current_row.is_empty() {
        rows.push(current_row);
    }
    rows
}

impl Base for TextInput {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.layout.set_pos(pos);
    }

    fn get_draw_pos(&self) -> (i32, i32) {
        self.layout.get_draw_pos()
    }

    fn draw(
        &self,
        draw_handle: &mut RaylibDrawHandle,
    ) -> Vec<AbsoluteDraw> {
        let layout = &self.layout;
        return self.layout.draw(draw_handle);
    }

    fn get_children(&self) -> Vec<Rc<RefCell<dyn Base>>> {
        self.layout.get_children()
    }

    fn get_mouse_event_handlers(&self, mouse_event: MouseEvent) -> Vec<String> {
        let layout = &self.layout;
        let mouse_pos = mouse_event.pos;
        let max_x = layout.pos.0 + layout.draw_dim.0;
        let max_y = layout.pos.1 + layout.draw_dim.1;
        if mouse_event.left_button_down
            && mouse_pos.0 as i32 >= layout.pos.0
            && mouse_pos.0 as i32 <= max_x
            && mouse_pos.1 as i32 >= layout.pos.1
            && mouse_pos.1 as i32 <= max_y
        {
            return vec![self.get_id()];
        } else {
            return vec![];
        }
    }

    fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>> {
        self.layout.get_on_click()
    }

    fn execute_on_click(&self, mouse_event: MouseEvent) -> bool {
        let mut user_fun = self.layout.on_click.borrow_mut();
        user_fun(mouse_event)
    }

    fn get_key_event_handlers(&self, _key_event: KeyEvent) -> Vec<String> {
        vec![self.get_id()]
    }

    fn get_on_key(&self) -> Rc<RefCell<dyn FnMut(KeyEvent) -> bool>> {
        self.def_on_key.clone()
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn set_raw_dim(&mut self, parent_draw_dim: (i32, i32)) {
        let layout = &mut self.layout;
        layout.children = vec![RawText::new(
            &self.content.borrow(),
            self.font_size,
            layout.padding,
            self.text_color,
        )];
        let content_width = unsafe {
            let c_text = CString::new(self.content.borrow().as_str()).unwrap();
            raylib::ffi::MeasureText(c_text.as_ptr(), self.font_size)
                + layout.padding.0
                + layout.padding.2
        };
        let (mut draw_width, mut draw_height) =
            crate::ui::common::get_draw_dim(layout.dim, parent_draw_dim, &layout.children, &layout.direction);

        if self.wrap {
            let max_width = draw_width - layout.padding.0 - layout.padding.2;
            if content_width > max_width {
                let text_rows = get_text_rows(&self.content.borrow(), max_width, self.font_size);
                layout.children = text_rows
                    .iter()
                    .map(|row| {
                        RawText::new(row, self.font_size, layout.padding, self.text_color)
                            as Rc<RefCell<dyn Base>>
                    })
                    .collect();
            }
        }

        if layout.dim.0 == Length::FIT {
            draw_width = layout
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().0)
                .max()
                .unwrap_or(0)
        }
        if layout.dim.1 == Length::FIT {
            draw_height = layout
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().1)
                .sum::<i32>()
                + layout.gap * (layout.children.len() as i32 - 1);
        }

        layout.draw_dim = (draw_width, draw_height);
    }

    fn get_draw_dim(&self) -> (i32, i32) {
        self.layout.get_draw_dim()
    }

    fn pass_1(&mut self, parent_draw_dim: (i32, i32), id: usize) -> usize {
        self.layout.pass_1(parent_draw_dim, id)
    }

    fn pass_2(&mut self, parent_pos: (i32, i32)) {
        self.layout.pass_2(parent_pos);
    }

    fn pass_overflow(&mut self, parent_draw_dim: (i32, i32), parent_pos: (i32, i32), scroll_map: &mut HashMap<String, i32>, y_offset: i32) {
        self.layout.pass_overflow(parent_draw_dim, parent_pos, scroll_map, y_offset);
    }

    fn get_overflow(&self) -> (bool, bool) {
        self.layout.get_overflow()
    }

    fn get_flex(&self) -> f32 {
        self.layout.get_flex()
    }

    fn debug_dims(&self, depth: usize) {
        let layout = &self.layout;
        tabbed_print(
            &format!(
                "<textinput width={} height={} x={} y={} padding=({},{},{},{}) gap={} dir={:?} main_align={:?} cross_align={:?} name='{}' flex={}>",
                layout.draw_dim.0,
                layout.draw_dim.1,
                layout.pos.0,
                layout.pos.1,
                layout.padding.0,
                layout.padding.1,
                layout.padding.2,
                layout.padding.3,
                layout.gap,
                layout.direction,
                layout.main_align,
                layout.cross_align,
                self.get_id(),
                layout.flex
            ),
            depth,
        );
        for (_i, child) in layout.children.iter().enumerate() {
            child.borrow().debug_dims(depth + 1);
        }
        tabbed_print("</textinput>", depth);
    }

    fn get_id(&self) -> String {
        self.layout.get_id()
    }

    fn get_by_id(&self, id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        self.layout.get_by_id(id)
    }

    fn get_position(&self) -> super::common::Position {
        self.layout.get_position()
    }
}
