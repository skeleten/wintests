#![allow(unused_imports, dead_code)]

use winapi::*;
use user32::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use std::mem;
use std::ffi::OsString;
use std::ops::Deref;

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

pub struct Color(u8, u8, u8, u8);

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

pub struct Font(HFONT);

impl Font {
    pub fn new(handle: HFONT) -> Self {
        Font(handle)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        let &mut Font(ref handle) = self;
        unsafe {
            DeleteObject(*handle as *mut c_void);
        }
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
