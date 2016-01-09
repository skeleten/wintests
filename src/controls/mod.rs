pub mod label;

use ::winapi::winuser::NMHDR;

pub trait Control {
    fn handle_notify(&mut self, info: *const NMHDR);
}
