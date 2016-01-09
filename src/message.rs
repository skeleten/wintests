use ::std::any::{ Any, TypeId };
use ::std::marker::Reflect;
use ::std::rc::Rc;
use ::winapi::*;

pub fn send_message<T>(hwnd: HWND, msg: T) {
    let msg = Box::into_raw(Box::new(msg)) as LPARAM;
    unsafe {::user32::SendMessageW(hwnd, WM_APP, 0, msg)};
}

pub trait MessageHandler<T> {
    fn handle_message(&mut self, &T);
}

pub trait MessageHandlerBase {
    fn register_message<T: Reflect>(&mut self)
        where Self: MessageHandler<T>;
    fn get_message_handler(&self, t: TypeId) ->Rc<Fn(&mut Self, Box<Any>)>;
    fn handle_message(&mut self, msg: Box<Any>) {
        let handler = self.get_message_handler(msg.get_type_id());
        handler(self, msg);
    }
}
