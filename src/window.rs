#![allow(non_upper_case_globals)]

use winapi::*;
use user32::*;
use kernel32::*;
use wio::wide::*;
use ::message::{ MessageHandlerBase, MessageHandler };

use ::ml::*;
use ::colors::*;

use std::any::{ Any, TypeId };
use std::collections::HashMap;
use std::marker::Reflect;
use std::ffi::OsString;
use std::rc::Rc;


// TODO: rewrite this, using constants in the `WindowClass` trait!
// also shouldn't really contain a WNDCLASSEXW, but the individual properties instead.
pub struct WindowClassBuilder<T: WindowClass> {
    class_name: String,
    style: UINT,
    icon: HICON,
    icon_small: HICON,
    cursor: HCURSOR,
    background_brush: HBRUSH,
    menu_name: String,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<T: WindowClass> WindowClassBuilder<T> {
    pub fn new() -> Self {
        WindowClassBuilder {
            class_name: T::class_name().into(),
            style: T::default_class_style(),
            icon: T::default_icon(),
            icon_small: T::default_icon_small(),
            menu_name: T::default_menu_name().into(),
            background_brush: T::default_background_brush(),
            cursor: T::default_cursor(),
            _phantom: ::std::marker::PhantomData,
        }
    }

    pub fn register(self, instance: HINSTANCE) {
        unsafe {
            let mut class_name = self.class_name.to_wide_null();
            let mut menu_name_raw = self.menu_name.to_wide_null();
            let menu_name = if self.menu_name.is_empty() {
                ::std::ptr::null_mut()
            } else {
                menu_name_raw.as_mut_ptr()
            };
            let mut wndclass: WNDCLASSEXW = ::std::mem::zeroed();
            wndclass.cbSize = ::std::mem::size_of::<WNDCLASSEXW>() as UINT;
            wndclass.style = self.style;
            wndclass.lpfnWndProc = Some(window_proc);
            wndclass.hInstance = instance;
            wndclass.hIcon = self.icon;
            wndclass.hIconSm = self.icon_small;
            wndclass.hCursor = self.cursor;
            wndclass.lpszClassName = class_name.as_mut_ptr();
            wndclass.lpszMenuName = menu_name;
            wndclass.hbrBackground = self.background_brush;
            RegisterClassExW(&wndclass);
        }
    }

    // TODO: setters for the variables
}

pub struct WindowBuilder<T: Window> {
    ex_style:   u32,
    class_name: String,
    window_name: String,
    style: u32,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    parent: HWND,
    menu: HMENU,
    hinstance: HINSTANCE,
    lp_param: LPVOID,
    _phantom: ::std::marker::PhantomData<T>,
}

pub fn get_window_from_handle<'a>(handle: &'a HWND) -> &'a Box<Window> {
    unsafe {
        let ptr = GetWindowLongPtrW(*handle, GWLP_USERDATA) as *mut Box<Window>;
        if ptr.is_null() {
            println!("Getting NULL as window from handle..");
        }
        &*ptr
    }
}

pub fn get_window_from_handle_mut<'a>(handle: &'a mut HWND) -> &'a mut Box<Window> {
    unsafe {
        let ptr = GetWindowLongPtrW(*handle, GWLP_USERDATA) as *mut Box<Window>;
        &mut *ptr
    }
}

impl<T: Window + WindowClass> WindowBuilder<T> {
    pub fn new(instance: HINSTANCE) -> Self {
        WindowBuilder {
            ex_style:   T::default_extended_style(),
            class_name: T::class_name().to_string(),
            window_name: T::default_title().to_string(),
            style: T::default_style(),
            pos_x: CW_USEDEFAULT,
            pos_y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            parent: ::std::ptr::null_mut(),
            menu: ::std::ptr::null_mut(),
            hinstance: instance,
            lp_param: ::std::ptr::null_mut(),
            _phantom: ::std::marker::PhantomData,
        }
    }

    pub fn build(self) -> Result<HWND, DWORD> {
        unsafe {
            let class_name = OsString::from(&self.class_name).to_wide_null();
            let window_name = OsString::from(&self.window_name).to_wide_null();
            let wnd_struct = Box::new(T::new());
            let handle = CreateWindowExW(
                self.ex_style,
                class_name.as_ptr(),
                window_name.as_ptr(),
                self.style,
                self.pos_x,
                self.pos_y,
                self.width,
                self.height,
                self.parent,
                self.menu,
                self.hinstance,
                Box::into_raw(wnd_struct) as LPVOID
            );

            if handle.is_null() {
                Err(GetLastError())
            } else {
                Ok(handle)
            }
        }
    }

    pub fn set_title<S: Into<String>>(mut self, name: S) -> Self {
        self.window_name = name.into();
        self
    }

    pub fn set_position(mut self, x: i32, y: i32) -> Self {
        self.pos_x = x;
        self.pos_y = y;
        self
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    pub fn set_parent(mut self, parent: HWND) -> Self {
        self.parent = parent;
        self
    }

    pub fn set_menu(mut self, menu: HMENU) -> Self {
        self.menu = menu;
        self
    }

    pub fn set_lp_param(mut self, lp_param: LPVOID) -> Self {
        self.lp_param = lp_param;
        self
    }
}

pub trait WindowClass {
    fn class_name() -> &'static str;
    fn default_title() -> &'static str;
    fn default_extended_style() -> DWORD { WS_EX_CLIENTEDGE }
    fn default_style() -> UINT { WS_TILEDWINDOW }
    fn default_class_style() -> UINT { CS_DROPSHADOW | CS_HREDRAW | CS_VREDRAW }
    fn default_icon() -> HICON {
        unsafe { LoadIconW(::std::ptr::null_mut(), IDI_APPLICATION) }
    }
    fn default_icon_small() -> HICON {
        unsafe { LoadIconW(::std::ptr::null_mut(), IDI_APPLICATION) }
    }
    fn default_menu_name() -> &'static str { "" }
    fn default_background_brush() -> HBRUSH {
        unsafe { ::gdi32::CreateSolidBrush(Color(0, 255, 255, 255).to_int()) }
    }
    fn default_cursor() -> HCURSOR {
        unsafe { LoadCursorW(::std::ptr::null_mut(), IDC_ARROW) }
    }

    fn new() -> Box<Window>;
}

pub trait Window : Paintable {
    fn init_handle(&mut self, handle: HWND);
    fn get_core<'a>(&'a self) -> &'a WindowCore;
    fn get_core_mut<'a>(&'a mut self) -> &'a mut WindowCore;
    fn on_create(&mut self) { }
    fn get_debug_name(&self) -> String { "Window Base".to_string() }
    fn get_unique_id(&mut self) -> i32 { self.get_core_mut().get_unique_id() }
}

pub struct WindowCore {
    handle:     HWND,
    controls:   Vec<Box<Paintable>>,
    handlers:   HashMap<::std::any::TypeId, Rc<Fn(&mut WindowCore, Box<Any>)>>,
    ids:        Box<Iterator<Item=i32>>,
}

impl WindowCore {
    pub fn from_handle(handle: HWND) -> Self {
        WindowCore {
            handle: handle,
            controls: Vec::new(),
            handlers: HashMap::new(),
            ids: Box::new((0..).into_iter()),
        }
    }

    pub fn add_control(&mut self, control: Box<Paintable>) {
        self.controls.push(control);
    }

    pub fn show(&self, cmd_show: i32) {
        unsafe {
            ShowWindow(self.handle, cmd_show);
        }
    }

    pub fn update(&self) {
        unsafe {
            UpdateWindow(self.handle);
        }
    }

    pub fn get_unique_id(&mut self) -> i32 {
        self.ids.next().unwrap()
    }
}

impl Paintable for WindowCore {
    fn paint(&self, context: &PaintContext) {
        for c in self.controls.iter().filter(|c| c.needs_repaint(context)) {
            c.paint(context);
        }
    }
}

impl MessageHandlerBase for WindowCore {
    fn register_message<T: Reflect + 'static>(&mut self)
            where Self: MessageHandler<T> {
        let func = Rc::new(|wnd: &mut Self, msg: Box<Any>| {
            let msg = msg.downcast_ref::<T>().unwrap();
            <Self as MessageHandler<T>>::handle_message(wnd, msg);
        });
        self.handlers.insert(TypeId::of::<T>(), func);
    }

    fn get_message_handler(&self, t: TypeId) -> Rc<Fn(&mut Self, Box<Any>)> {
        self.handlers.get(&t).unwrap().clone()
    }
}

#[allow(unused_variables)]
unsafe extern "system" fn window_proc(mut hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        // TODO: override *all* the messages.
        WM_CREATE => {
            let st = l_param as *mut CREATESTRUCTW;
            if l_param == 0 || (*st).lpCreateParams.is_null() {
                println!("Create params are null!");
            }
            let mut wnd = Box::from_raw((*st).lpCreateParams as *mut Box<Window>);
            wnd.init_handle(hwnd);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(wnd) as LONG_PTR);
            get_window_from_handle_mut(&mut hwnd).on_create();
            0
        },
        WM_CLOSE => {
            println!("WM_CLOSE");
            DestroyWindow(hwnd);
            0
        },
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        },
        WM_PAINT =>  {
            0
        },
        msg => DefWindowProcW(hwnd, msg, w_param, l_param)
    }
}
