#![allow(non_upper_case_globals)]

use winapi::*;
use user32::*;

use std::mem;
use std::ops::{ Deref };

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

pub trait Paintable {
    fn paint(&self, c: &PaintContext);
    fn needs_repaint(&self, _c: &PaintContext) -> bool {
        true
    }
}
