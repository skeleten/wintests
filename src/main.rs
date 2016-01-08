#![allow(dead_code)]

extern crate winapi;
extern crate user32;
extern crate gdi32;
extern crate kernel32;
extern crate wio;
#[macro_use] extern crate bitflags;

mod ml;
mod colors;
mod font;
mod controls;


use winapi::*;
use user32::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use ml::*;
use colors::*;
use controls::label::Label;

struct MyWindow {
    core: Option<WindowCore>,
}

impl Window for MyWindow {
    fn get_core<'a>(&'a self) -> &'a WindowCore {
        &self.core.as_ref().unwrap()
    }

    fn get_core_mut<'a>(&'a mut self) -> &'a mut WindowCore {
        self.core.as_mut().unwrap()
    }

    fn init_handle(&mut self, handle: HWND) {
        self.core = Some(WindowCore::from_handle(handle))
    }

    fn on_create(&mut self) {
        let mut lbl = Label::new();
        lbl.font_builder.set_height(24).set_face("Fira Code");
        lbl.text = ">- Test -<".to_string();
        self.get_core_mut().add_control(Box::new(lbl));
    }
}

impl Paintable for MyWindow {
    fn paint(&self, context: &PaintContext) {
        println!("Painting MyWindow!");
        self.get_core().paint(context);
    }
}

impl WindowClass for MyWindow {
    fn class_name() -> &'static str { "myWindowClass" }
    fn default_title() -> &'static str { "My Window Title" }

    fn new() -> Box<Window> {
        Box::new(MyWindow {
            core: None,
        })
    }
}

pub fn main() {
    println!("common main :(");
    unsafe {
        let modhandle = GetModuleHandleW(std::ptr::null_mut());
        WinMain(modhandle, std::ptr::null_mut(), std::ptr::null_mut(), SW_SHOW);
    }
}

#[no_mangle] #[allow(non_snake_case, unused_variables)]
pub unsafe extern "system" fn WinMain(hinstance: HINSTANCE,
                                      prevInstance: HINSTANCE,
                                      cmdLine: LPSTR,
                                      cmdShow: i32,
                                    ) -> u32 {
    let mut classname = MyWindow::class_name().to_wide_null();
    WindowClassBuilder::new()
        .set_style(HRedraw | VRedraw)
        .set_procedure(Some(wndProc))
        .set_hinstance(hinstance)
        .set_icon(LoadIconW(std::ptr::null_mut(), IDI_APPLICATION))
        .set_background_brush(CreateSolidBrush(Color(0, 25, 50, 75).to_int()))
        .set_class_name(classname.as_mut_ptr())
        .set_icon_small(LoadIconW(std::ptr::null_mut(), IDI_APPLICATION))
        .set_cursor(LoadCursorW(std::ptr::null_mut(), IDC_ARROW))
        .register().expect("Could't register window class.");
    println!("Registered window.");
    let hwnd = match WindowBuilder::<MyWindow>::new(hinstance)
                .set_title("Test Window")
                .set_width(400).set_height(400)
                .build() {
        Ok(handle) => handle,
        Err(e) => {
            println!("HWND is NULL! ({})", e);
            return 0xFFFF_FFFF;
        }
    };
    println!("Created Window.");
    ShowWindow(hwnd, cmdShow);
    UpdateWindow(hwnd);

    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);
    };

    return 0;
}

fn repaint_window(context: &PaintContext) {
    let window: &Box<Window> = get_window_from_handle(context.window);
    // TODO FIXME: This isn't actually called/dispatched for some weird reason
    window.paint(context);
}

fn destroy_window(handle: HWND) {
    // TODO FIXME: This doesn't actually return for some reason.
    let window = unsafe {
        Box::from_raw(GetWindowLongPtrW(handle, GWLP_USERDATA) as *mut Box<Window>)
    };
    drop(window);
}

#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn wndProc(hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match msg {
        // TODO: override *all* the messages.
        WM_CREATE => {
            let st = lParam as *mut CREATESTRUCTW;
            if lParam == 0 || (*st).lpCreateParams.is_null() {
                println!("Create params are null!");
            }
            let mut wnd = Box::from_raw((*st).lpCreateParams as *mut Box<Window>);
            wnd.init_handle(hwnd);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, (*st).lpCreateParams as LONG_PTR);
            wnd.on_create();
            0
        },
        WM_CLOSE => {
            println!("WM_CLOSE");
            DestroyWindow(hwnd);
            0
        },
        WM_DESTROY => {
            // TODO: FIX THIS FUNCTION
            // destroy_window(hwnd);
            PostQuitMessage(0);
            0
        },
        WM_PAINT =>  {
            repaint_window(&PaintContext::begin_paint(&hwnd));
            0
        },
        msg => DefWindowProcW(hwnd, msg, wParam, lParam)
    }
}
