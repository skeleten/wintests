#![allow(non_upper_case_globals)]

use winapi::*;
use user32::*;
use kernel32::*;
use wio::wide::*;

use std::mem;
use std::ffi::OsString;
use std::ops::{ Deref, DerefMut };

pub struct DeviceContext<'a> {
    window: &'a HWND,
    context: HDC,
}

impl<'a> DeviceContext<'a> {
    pub fn from_hwnd(hwnd: &'a HWND) -> Self {
        unsafe {
            DeviceContext {
                window: hwnd,
                context: GetDC(*hwnd),
            }
        }
    }
}

impl<'a> Drop for DeviceContext<'a> {
    fn drop(&mut self) {
        unsafe {
            ReleaseDC(*self.window, self.context);
        }
    }
}

impl<'a> Deref for DeviceContext<'a> {
    type Target = HDC;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

pub struct PaintContext<'a> {
    pub window: &'a HWND,
    pub context: HDC,
    pub paintstruct: PAINTSTRUCT,
}

impl<'a> PaintContext<'a> {
    pub fn begin_paint(handle: &'a HWND) -> Self {
        unsafe {
            let mut ps = mem::zeroed();
            let hdc = BeginPaint(*handle, &mut ps);
            PaintContext {
                window: handle,
                context: hdc,
                paintstruct: ps,
            }
        }
    }
}

impl<'a> Drop for PaintContext<'a> {
    fn drop(&mut self) {
        unsafe {
            EndPaint(*self.window, &mut self.paintstruct);
        }
    }
}

impl<'a> Deref for PaintContext<'a> {
    type Target = HDC;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

bitflags!{
    flags WindowStyle: DWORD {
        const ByteAlignClient       = CS_BYTEALIGNCLIENT,
        const ByteAlignWindow       = CS_BYTEALIGNWINDOW,
        const ClassDC               = CS_CLASSDC,
        const DoubleClicks          = CS_DBLCLKS,
        const Dropshadow            = CS_DROPSHADOW,
        const GlobalClass           = CS_GLOBALCLASS,
        const HRedraw               = CS_HREDRAW,
        const NoClose               = CS_NOCLOSE,
        const OwnDC                 = CS_OWNDC,
        const ParentDC              = CS_PARENTDC,
        const SaveBits              = CS_SAVEBITS,
        const VRedraw               = CS_VREDRAW,
    }
}

pub struct WindowClassBuilder(WNDCLASSEXW);

impl WindowClassBuilder {
    pub fn new() -> Self {
        unsafe {
            let mut wnd = mem::zeroed::<WNDCLASSEXW>();
            wnd.cbSize = mem::size_of::<WNDCLASSEXW>() as DWORD;
            WindowClassBuilder(wnd)
        }
    }

    pub fn set_style(self, style: WindowStyle) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.style = style.bits();
        WindowClassBuilder(wnd)
    }

    pub fn set_procedure(self, prc: WNDPROC) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.lpfnWndProc = prc;
        WindowClassBuilder(wnd)
    }

    pub fn set_class_extra(self, amount: i32) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.cbClsExtra = amount;
        WindowClassBuilder(wnd)
    }

    pub fn set_window_extra(self, amount: i32) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.cbWndExtra = amount;
        WindowClassBuilder(wnd)
    }

    pub fn set_hinstance(self, instance: HINSTANCE) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.hInstance = instance;
        WindowClassBuilder(wnd)
    }

    pub fn set_icon(self, icon: HICON) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.hIcon = icon;
        WindowClassBuilder(wnd)
    }

    pub fn set_cursor(self, cursor: HCURSOR) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.hCursor = cursor;
        WindowClassBuilder(wnd)
    }

    pub fn set_background_brush(self, brush: HBRUSH) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.hbrBackground = brush;
        WindowClassBuilder(wnd)
    }

    pub fn set_menu_name(self, name: LPCWSTR) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.lpszMenuName = name;
        WindowClassBuilder(wnd)
    }

    pub fn set_class_name(self, name: LPCWSTR) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.lpszClassName = name;
        WindowClassBuilder(wnd)
    }

    pub fn set_icon_small(self, icon: HICON) -> Self {
        let WindowClassBuilder(mut wnd) = self;
        wnd.hIconSm = icon;
        WindowClassBuilder(wnd)
    }

    pub fn register(self) -> Result<(), DWORD> {
        let WindowClassBuilder(mut wnd) = self;
        unsafe {
            if RegisterClassExW(&mut wnd) == 0 {
                Err(GetLastError())
            } else {
                Ok(())
            }
        }
    }
}

pub trait Paintable {
    fn paint(&self, c: &PaintContext);
    fn needs_repaint(&self, _c: &PaintContext) -> bool {
        true
    }
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

impl<T: Window> WindowBuilder<T> {
    pub fn new(instance: HINSTANCE) -> Self {
        WindowBuilder {
            ex_style:   T::default_extended_style(),
            class_name: T::class_name(),
            window_name: T::default_window_name(),
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
                self.lp_param
            );

            if handle.is_null() {
                Err(GetLastError())
            } else {
                let window = Box::new(Box::new(WindowCore::from_handle(handle)));
                SetWindowLongPtrW(handle, GWLP_USERDATA, Box::into_raw(window) as LONG_PTR);
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

pub trait Window : Deref<Target = WindowCore> + DerefMut {
    fn from_handle(handle: HWND) -> Self;
    fn class_name() -> String;
    fn default_window_name() -> String {
        "Window".to_string()
    }
    fn default_extended_style() -> u32 {
        WS_EX_CLIENTEDGE
    }
    fn default_style() -> u32 {
        WS_OVERLAPPEDWINDOW
    }

}

pub struct WindowCore {
    handle:     HWND,
    controls:   Vec<Box<Paintable>>,
}

impl WindowCore {
    pub fn from_handle(handle: HWND) -> Self {
        WindowCore {
            handle: handle,
            controls: Vec::new(),
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
}

impl Paintable for WindowCore {
    fn paint(&self, context: &PaintContext) {
        for c in self.controls.iter().filter(|c| c.needs_repaint(context)) {
            c.paint(context);
        }
    }
}
