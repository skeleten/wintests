#![feature(link_args)]
#![allow(dead_code)]

extern crate winapi;
extern crate user32;
extern crate gdi32;
extern crate kernel32;
extern crate wio;

use winapi::*;
use user32::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use std::ffi::OsString;
use std::ops::Deref;

// Link as "Windows application" to avoid console window flash
#[link_args = "-Wl,--subsystem,windows"]
extern {}

struct DeviceContext<'a> {
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

struct PaintContext<'a> {
    window: &'a HWND,
    context: HDC,
    paintstruct: PAINTSTRUCT,
}

impl<'a> PaintContext<'a> {
    pub fn begin_paint(handle: &'a HWND) -> Self {
        unsafe {
            let mut ps = std::mem::zeroed();
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

struct Color(u8, u8, u8, u8);
impl Color {
    pub fn from_rbg(r: u8, g: u8, b: u8) -> Self {
        Color(0, r, g, b)
    }

    pub fn from_int(i: u32) -> Self {
        let a = ((i >> 24) & 255) as u8;
        let r = ((i >> 16) & 255) as u8;
        let g = ((i >> 8) & 255) as u8;
        let b = (i & 255) as u8;
        Color(a, r, g, b)
    }

    pub fn to_int(self) -> u32 {
        let Color(a, r, g, b) = self;

        (b as u32)        |
        (g as u32) << 8   |
        (r as u32) << 16  |
        (a as u32) << 24
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
pub unsafe extern "system" fn WinMain(hinstance: HINSTANCE, prevInstance: HINSTANCE, cmdLine: LPSTR, cmdShow: i32) -> u32 {
    let mut classname: Vec<u16> = OsString::from("myWindowClass").to_wide_null();
    let mut wndtitle: Vec<u16> = OsString::from("Title").to_wide_null();
    let wc = winuser::WNDCLASSEXW {
        cbSize: 48, //??? sizeof(winuser::WINDCLASSEXW),
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndProc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: LoadIconW(std::ptr::null_mut(), IDI_APPLICATION),
        hbrBackground: CreateSolidBrush(Color::from_rbg(50, 50, 50).to_int()),
        lpszMenuName: std::ptr::null_mut(),
        lpszClassName: classname.as_mut_ptr(),
        hIconSm: LoadIconW(std::ptr::null_mut(), IDI_APPLICATION),
        hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
    };

    if RegisterClassExW(&wc) == 0 {
        println!("couldn't register class.. {}",  GetLastError());
    } else { println!("registered class!"); }


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
