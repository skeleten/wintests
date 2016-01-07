#![allow(dead_code)]

extern crate winapi;
extern crate user32;
extern crate gdi32;
extern crate kernel32;
extern crate wio;
#[macro_use] extern crate bitflags;

mod ml;
mod colors;

use winapi::*;
use user32::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use ml::*;
use colors::*;
use std::ffi::OsString;

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
    let mut classname: Vec<u16> = OsString::from("myWindowClass").to_wide_null();
    WindowClassBuilder::new()
        .set_style(HRedraw | VRedraw)
        .set_procedure(Some(wndProc))
        .set_hinstance(hinstance)
        .set_icon(LoadIconW(std::ptr::null_mut(), IDI_APPLICATION))
        .set_background_brush(CreateSolidBrush(colors::DARK_GRAY.to_int()))
        .set_class_name(classname.as_mut_ptr())
        .set_icon_small(LoadIconW(std::ptr::null_mut(), IDI_APPLICATION))
        .set_cursor(LoadCursorW(std::ptr::null_mut(), IDC_ARROW))
        .register().expect("Could't register window class.");

    let hwnd = WindowBuilder::new(hinstance, "myWindowClass".to_string())
                .set_title("Test Window".to_string())
                .set_width(400).set_height(400)
                .build().ok().unwrap();

    if hwnd.is_null() {
        println!("hwnd is null ({})", GetLastError());
    }

    let mut lbl = Box::new(Label::new());
    lbl.font_builder.set_height(24).set_face("Fira Code".to_string());
    lbl.text =">- Test -<".to_string();
    lbl.foreground_color = colors::WHITE;

    get_window_from_handle_mut(&hwnd).add_control(lbl);

    ShowWindow(hwnd, cmdShow);
    UpdateWindow(hwnd);

    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);
    };

    return 0;
}

unsafe fn get_window_from_handle<'a>(handle: &'a HWND) -> &'a Window {
    let ptr = GetWindowLongPtrW(*handle, GWLP_USERDATA) as *mut Window;
    &*ptr
}

unsafe fn get_window_from_handle_mut<'a>(handle: &'a HWND) -> &'a mut Window {
    let ptr = GetWindowLongPtrW(*handle, GWLP_USERDATA) as *mut Window;
    &mut *ptr
}

fn repaint_window(context: &PaintContext) {
    let window = unsafe {
        let ptr = GetWindowLongPtrW(*context.window, GWLP_USERDATA) as *mut Window;
        &*ptr
    };
    window.paint(context);
}

fn destroy_window(handle: HWND) {
    let window = unsafe { Box::from_raw(GetWindowLongPtrW(handle, GWLP_USERDATA) as *mut Window) };
    ::std::mem::drop(window);
}

#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn wndProc(hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd);
            0
        },
        WM_DESTROY => {
            destroy_window(hwnd);
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
