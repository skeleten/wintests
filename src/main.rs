#![allow(dead_code)]

extern crate winapi;
extern crate user32;
extern crate gdi32;
extern crate kernel32;
extern crate wio;
#[macro_use] extern crate bitflags;

mod ml;

use winapi::*;
use user32::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use ml::*;

use std::ffi::OsString;
use std::ops::Deref;


pub fn main() {
    println!("common main :(");
    unsafe {
        let modhandle = GetModuleHandleW(std::ptr::null_mut());
        WinMain(modhandle, std::ptr::null_mut(), std::ptr::null_mut(), SW_SHOW);
    }
}

#[no_mangle] #[allow(non_snake_case, unused_variables)]
pub unsafe extern "system" fn WinMain(hinstance: HINSTANCE, prevInstance: HINSTANCE, cmdLine: LPSTR, cmdShow: i32) -> u32 {
    let mut classname: Vec<u16> = OsString::from("myWindowClass").to_wide_null();
    let mut wndtitle: Vec<u16> = OsString::from("Title").to_wide_null();
    WindowClassBuilder::new()
        .set_style(HRedraw | VRedraw)
        .set_procedure(Some(wndProc))
        .set_hinstance(hinstance)
        .set_icon(LoadIconW(std::ptr::null_mut(), IDI_APPLICATION))
        .set_background_brush(CreateSolidBrush(Color::from_rbg(50, 50, 50).to_int()))
        .set_class_name(classname.as_mut_ptr())
        .set_icon_small(LoadIconW(std::ptr::null_mut(), IDI_APPLICATION))
        .set_cursor(LoadCursorW(std::ptr::null_mut(), IDC_ARROW))
        .register().expect("Could't register window class.");

    let hwnd = CreateWindowExW(
        WS_EX_CLIENTEDGE,
        classname.as_mut_ptr(),
        wndtitle.as_mut_ptr(),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT,
        400, 400,
        std::ptr::null_mut(), std::ptr::null_mut(),
        hinstance, std::ptr::null_mut());
    if hwnd.is_null() {
        println!("hwnd is null ({})", GetLastError());
    }

    ShowWindow(hwnd, cmdShow);
    UpdateWindow(hwnd);

    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);
    };

    return 0;

}

unsafe fn draw_message(context: &PaintContext) {
    /* Draw a string */
    let font_name = OsString::from("Fira Code").to_wide_null();
    let message = OsString::from("-> ~=Test <-").to_wide_null();

    let font = CreateFontW(24, 0, 0, 0,
        FW_DONTCARE,
        FALSE as DWORD,
        FALSE as DWORD,
        FALSE as DWORD,
        DEFAULT_CHARSET,
        OUT_DEFAULT_PRECIS,
        CLIP_DEFAULT_PRECIS,
        DEFAULT_QUALITY,
        DEFAULT_PITCH,
        font_name.as_ptr());

    let mut rect = std::mem::zeroed();
    SelectObject(**context, font as *mut c_void);
    SetTextColor(**context, Color::from_rbg(255, 255, 255).to_int());
    GetClientRect(*context.window, &mut rect);
    SetBkMode(**context, TRANSPARENT);
    rect.left= 40;
    rect.top = 10;
    DrawTextW(**context, message.as_ptr(), -1, &mut rect, DT_SINGLELINE | DT_NOCLIP);
}

unsafe fn repaint_window(context: &PaintContext) {
    draw_message(context);
}

#[no_mangle] #[allow(non_snake_case, unused_variables)]
pub unsafe extern "system" fn wndProc(hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd);
            0
        },
        WM_DESTROY => {
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
