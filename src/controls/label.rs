use winapi::*;
use user32::*;
use gdi32::*;
use wio::wide::*;

// NOTE: Do I actually have to fix these if I convert
// To using a library instead of an binary crate?
use ::ml::{ Paintable, PaintContext };
use ::colors::{ Color, BLACK };
use ::font::{ FontBuilder };

use super::Control;

pub struct Label {
    pub pos_x: usize,
    pub pos_y: usize,
    // width/height is questionable
    pub width: usize,
    pub height: usize,
    pub font_builder: FontBuilder,
    pub text: String,
    pub foreground_color: Color,
    // background color?
    // TODO: Format options
}

impl Label {
    pub fn new() -> Self {
        Label {
            pos_x: 0,
            pos_y: 0,
            width: 0,
            height: 0,
            font_builder: FontBuilder::new(),
            text: String::new(),
            foreground_color: BLACK,
        }
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.pos_x = x;
        self.pos_y = y;
    }
}

impl Paintable for Label {
    fn paint(&self, context: &PaintContext) {
        let text = self.text.to_wide_null();
        let font = self.font_builder.build().ok().unwrap();
        unsafe {
            let old_font = SelectObject(**context, *font as *mut c_void);
            SetTextColor(**context, self.foreground_color.to_int());
            let mut rect = RECT {
                left: self.pos_x as i32,
                top: self.pos_y as i32,
                right: (self.pos_x + self.width) as i32,
                bottom: (self.pos_y + self.height) as i32,
            };
            SetBkMode(**context, TRANSPARENT);
            DrawTextW(**context, text.as_ptr(), -1, &mut rect, DT_NOCLIP);
            SelectObject(**context, old_font);
        }
    }

    fn needs_repaint(&self, context: &PaintContext) -> bool {
        // return true;
                self.pos_x <= context.paintstruct.rcPaint.right as usize
            &&  self.pos_x >= context.paintstruct.rcPaint.left as usize
        ||      self.pos_y <= context.paintstruct.rcPaint.bottom as usize
            &&  self.pos_y >= context.paintstruct.rcPaint.top as usize
        ||      (self.pos_x + self.width) <= context.paintstruct.rcPaint.right as usize
            &&  (self.pos_x + self.width) >= context.paintstruct.rcPaint.left as usize
        ||      (self.pos_y + self.height) <= context.paintstruct.rcPaint.bottom as usize
            &&  (self.pos_y + self.height) >= context.paintstruct.rcPaint.top as usize
    }
}

impl Control for Label {
    #[allow(unused_variables)]
    fn handle_notify(&mut self, info: *const NMHDR) {
        // This shouldn't ever happen!
        unimplemented!();
    }
}
