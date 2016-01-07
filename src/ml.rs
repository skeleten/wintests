#![allow(unused_imports, dead_code, non_upper_case_globals)]

use winapi::*;
use user32::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use ::colors::*;

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

#[derive(Clone, Debug)]
pub enum FontWeight {
    DontCare,
    Thin,
    ExtraLight,
    UltraLight,
    Light,
    Normal,
    Regular,
    Medium,
    Semibold,
    Demibold,
    Bold,
    ExtraBold,
    UltraBold,
    Heavy,
    Black,
    Other(i32)
}

impl FontWeight {
    pub fn to_int(&self) -> i32 {
        match *self {
            FontWeight::DontCare        => FW_DONTCARE,
            FontWeight::Thin            => FW_THIN,
            FontWeight::ExtraLight      => FW_EXTRALIGHT,
            FontWeight::UltraLight      => FW_ULTRALIGHT,
            FontWeight::Light           => FW_LIGHT,
            FontWeight::Normal          => FW_NORMAL,
            FontWeight::Regular         => FW_REGULAR,
            FontWeight::Medium          => FW_MEDIUM,
            FontWeight::Semibold        => FW_SEMIBOLD,
            FontWeight::Demibold        => FW_DEMIBOLD,
            FontWeight::Bold            => FW_BOLD,
            FontWeight::ExtraBold       => FW_EXTRABOLD,
            FontWeight::UltraBold       => FW_ULTRABOLD,
            FontWeight::Heavy           => FW_HEAVY,
            FontWeight::Black           => FW_BLACK,
            FontWeight::Other(x)        => x
        }
    }
}

pub enum CharSet {
    Ansi,
    Baltic,
    ChineseBig5,
    Default,
    EastEurope,
    Gb2312,
    Greek,
    Hangul,
    Mac,
    OEM,
    Russian,
    ShiftJIS,
    Symbol,
    Turkish,
    Vietnamese,
    Johab,
    Arabic,
    Hebrew,
    Thai,
}

impl CharSet {
    pub fn to_uint(&self) -> u32 {
        match *self {
            CharSet::Ansi               => ANSI_CHARSET,
            CharSet::Baltic             => BALTIC_CHARSET,
            CharSet::ChineseBig5        => CHINESEBIG5_CHARSET,
            CharSet::Default            => DEFAULT_CHARSET,
            CharSet::EastEurope         => EASTEUROPE_CHARSET,
            CharSet::Gb2312             => GB2312_CHARSET,
            CharSet::Greek              => GREEK_CHARSET,
            CharSet::Hangul             => HANGUL_CHARSET,
            CharSet::Mac                => MAC_CHARSET,
            CharSet::OEM                => OEM_CHARSET,
            CharSet::Russian            => RUSSIAN_CHARSET,
            CharSet::ShiftJIS           => SHIFTJIS_CHARSET,
            CharSet::Symbol             => SYMBOL_CHARSET,
            CharSet::Turkish            => TURKISH_CHARSET,
            CharSet::Vietnamese         => VIETNAMESE_CHARSET,
            CharSet::Johab              => JOHAB_CHARSET,
            CharSet::Arabic             => ARABIC_CHARSET,
            CharSet::Hebrew             => HEBREW_CHARSET,
            CharSet::Thai               => THAI_CHARSET,
        }
    }
}

pub enum OutputPrecision {
    Character,
    Default,
    Device,
    Outline,
    PsOnly,
    Raster,
    String,
    Stroke,
    TrueTypeOnly,
    TrueType,
}

impl OutputPrecision {
    pub fn to_uint(&self) -> u32 {
        match *self {
            OutputPrecision::Character              => OUT_CHARACTER_PRECIS,
            OutputPrecision::Default                => OUT_DEFAULT_PRECIS,
            OutputPrecision::Device                 => OUT_DEVICE_PRECIS,
            OutputPrecision::Outline                => OUT_OUTLINE_PRECIS,
            OutputPrecision::PsOnly                 => OUT_PS_ONLY_PRECIS,
            OutputPrecision::Raster                 => OUT_RASTER_PRECIS,
            OutputPrecision::String                 => OUT_STRING_PRECIS,
            OutputPrecision::Stroke                 => OUT_STROKE_PRECIS,
            OutputPrecision::TrueTypeOnly           => OUT_TT_ONLY_PRECIS,
            OutputPrecision::TrueType               => OUT_TT_PRECIS,
        }
    }
}

pub enum ClipPrecision {
    Character,
    Default,
    DFADisable,
    Embedded,
    LHAngles,
    Mask,
    DFAOverride,
    Stroke,
    TrueTypeAlways,
}

impl ClipPrecision {
    pub fn to_uint(&self) -> u32 {
        match *self {
            ClipPrecision::Character        => CLIP_CHARACTER_PRECIS,
            ClipPrecision::Default          => CLIP_DEFAULT_PRECIS,
            ClipPrecision::DFADisable       => CLIP_DFA_DISABLE,
            ClipPrecision::Embedded         => CLIP_EMBEDDED,
            ClipPrecision::LHAngles         => CLIP_LH_ANGLES,
            ClipPrecision::Mask             => CLIP_MASK,
            ClipPrecision::Stroke           => CLIP_STROKE_PRECIS,
            ClipPrecision::TrueTypeAlways   => CLIP_TT_ALWAYS,
            // TODO: MISSING CONSTANT
            ClipPrecision::DFAOverride      => CLIP_DEFAULT_PRECIS,
        }
    }
}

pub enum FontQuality {
    AntiAliased,
    ClearType,
    Default,
    Draft,
    NonAntiAliased,
    Proof,
}

impl FontQuality {
    pub fn to_uint(&self) -> u32 {
        match *self {
            FontQuality::AntiAliased    => ANTIALIASED_QUALITY,
            FontQuality::ClearType      => CLEARTYPE_QUALITY,
            FontQuality::Default        => DEFAULT_QUALITY,
            FontQuality::Draft          => DRAFT_QUALITY,
            FontQuality::NonAntiAliased => NONANTIALIASED_QUALITY,
            FontQuality::Proof          => PROOF_QUALITY,
        }
    }
}

pub enum FontPitch {
    Decorative,
    DontCare,
    Modern,
    Roman,
    Script,
    Swiss,
}

impl FontPitch {
    pub fn to_uint(&self) -> u32 {
        match *self {
            // TODO: MISSING CONSTANTS :(
            _ => 0,
            /*
            FontPitch::Decorative   => FF_DECORATIVE,
            FontPitch::DontCare     => FF_DONTCARE,
            FontPitch::Modern       => FF_MODERN,
            FontPitch::Roman        => FF_ROMAN,
            FontPitch::Script       => FF_SCRIPT,
            FontPitch::Swiss        => FF_SWISS,
            */
        }
    }
}

pub struct FontBuilder {
    height: i32,
    width: i32,
    escapement: i32,
    orientation: i32,
    weight: FontWeight,
    italic: bool,
    underline: bool,
    strikeout: bool,
    charset: CharSet,
    output_precision: OutputPrecision,
    clip_precision: ClipPrecision,
    quality: FontQuality,
    pitch: FontPitch,
    face: String,
}

impl FontBuilder {
    pub fn new() -> Self {
        FontBuilder {
            height:             12,
            width:              0,
            escapement:         0,
            orientation:        0,
            weight:             FontWeight::DontCare,
            italic:             false,
            underline:          false,
            strikeout:          false,
            charset:            CharSet::Default,
            output_precision:   OutputPrecision::Default,
            clip_precision:     ClipPrecision::Default,
            quality:            FontQuality::Default,
            pitch:              FontPitch::DontCare,
            face:               String::new()
        }
    }

    pub fn build(&self) -> Result<Font, DWORD> {
        unsafe {
            let mut face_wide = OsString::from(&self.face).to_wide_null();
            let italic = if self.italic { TRUE as DWORD } else { FALSE as DWORD };
            let underline = if self.underline { TRUE as DWORD } else { FALSE as DWORD };
            let strikeout = if self.strikeout { TRUE as DWORD } else { FALSE as DWORD };

            let handle = CreateFontW(
                self.height,
                self.width,
                self.escapement,
                self.orientation,
                self.weight.to_int(),
                italic,
                underline,
                strikeout,
                self.charset.to_uint(),
                self.output_precision.to_uint(),
                self.clip_precision.to_uint(),
                self.quality.to_uint(),
                self.pitch.to_uint(),
                face_wide.as_mut_ptr());

            if handle.is_null() {
                Err(GetLastError())
            } else {
                Ok(Font::new(handle))
            }
        }
    }

    pub fn set_height(&mut self, height: i32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_width(&mut self, width: i32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_escapement(&mut self, escapement: i32) -> &mut Self {
        self.escapement = escapement;
        self
    }

    pub fn set_orientation(&mut self, orientation: i32) -> &mut Self {
        self.orientation = orientation;
        self
    }

    pub fn set_weight(&mut self, weight: FontWeight) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn set_italic(&mut self, italic: bool) -> &mut Self {
        self.italic = italic;
        self
    }

    pub fn set_underline(&mut self, underline: bool) -> &mut Self {
        self.underline = underline;
        self
    }

    pub fn set_strikeout(&mut self, strikeout: bool) -> &mut Self {
        self.strikeout = strikeout;
        self
    }

    pub fn set_charset(&mut self, charset: CharSet) -> &mut Self {
        self.charset = charset;
        self
    }

    pub fn set_output_precision(&mut self, precision: OutputPrecision) -> &mut Self {
        self.output_precision = precision;
        self
    }

    pub fn set_clip_precision(&mut self, precision: ClipPrecision) -> &mut Self {
        self.clip_precision = precision;
        self
    }

    pub fn set_quality(&mut self, quality: FontQuality) -> &mut Self {
        self.quality = quality;
        self
    }

    pub fn set_pitch(&mut self, pitch: FontPitch) -> &mut Self {
        self.pitch = pitch;
        self
    }

    pub fn set_face(&mut self, face: String) -> &mut Self {
        self.face = face;
        self
    }
}

pub struct Font(HFONT);

impl Font {
    pub fn new(handle: HFONT) -> Self {
        Font(handle)
    }
}

impl Deref for Font {
    type Target = HFONT;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        let &Font(ref handle) = self;
        handle
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

pub trait Paintable {
    fn paint(&self, c: &PaintContext);
    fn needs_repaint(&self, _c: &PaintContext) -> bool {
        true
    }
}

pub struct Label {
    pub pos_x: usize,
    pub pos_y: usize,
    // width/height is questionable
    pub width: usize,
    pub height: usize,
    pub font_builder: FontBuilder,
    pub text: String,
    pub foreground_color: Color,
    // background color?
    // TODO: Format options
}

impl Label {
    pub fn new() -> Self {
        Label {
            pos_x: 0,
            pos_y: 0,
            width: 0,
            height: 0,
            font_builder: FontBuilder::new(),
            text: String::new(),
            foreground_color: BLACK,
        }
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.pos_x = x;
        self.pos_y = y;
    }
}

impl Paintable for Label {
    fn paint(&self, context: &PaintContext) {
        let text = OsString::from(&self.text).to_wide_null();
        let font = self.font_builder.build().ok().unwrap();
        unsafe {
            let old_font = SelectObject(**context, *font as *mut c_void);
            SetTextColor(**context, self.foreground_color.to_int());
            let mut rect = RECT {
                left: self.pos_x as i32,
                top: self.pos_y as i32,
                right: (self.pos_x + self.width) as i32,
                bottom: (self.pos_y + self.height) as i32,
            };
            SetBkMode(**context, TRANSPARENT);
            DrawTextW(**context, text.as_ptr(), -1, &mut rect, DT_NOCLIP);
            SelectObject(**context, old_font);
        }
    }

    fn needs_repaint(&self, context: &PaintContext) -> bool {
                self.pos_x <= context.paintstruct.rcPaint.right as usize
            &&  self.pos_x >= context.paintstruct.rcPaint.left as usize
        ||      self.pos_y <= context.paintstruct.rcPaint.bottom as usize
            &&  self.pos_y >= context.paintstruct.rcPaint.top as usize
        ||      (self.pos_x + self.width) <= context.paintstruct.rcPaint.right as usize
            &&  (self.pos_x + self.width) >= context.paintstruct.rcPaint.left as usize
        ||      (self.pos_y + self.height) <= context.paintstruct.rcPaint.bottom as usize
            &&  (self.pos_y + self.height) >= context.paintstruct.rcPaint.top as usize
    }
}

pub struct WindowBuilder {
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
    lp_param: LPVOID
}

impl WindowBuilder {
    pub fn new(instance: HINSTANCE, class_name: String) -> Self {
        WindowBuilder {
            ex_style:   WS_EX_CLIENTEDGE,
            class_name: class_name,
            window_name: String::new(),
            style: WS_OVERLAPPEDWINDOW,
            pos_x: CW_USEDEFAULT,
            pos_y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            parent: ::std::ptr::null_mut(),
            menu: ::std::ptr::null_mut(),
            hinstance: instance,
            lp_param: ::std::ptr::null_mut(),
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
                let window = Box::new(Window::from_handle(handle));
                SetWindowLongPtrW(handle, GWLP_USERDATA, Box::into_raw(window) as LONG_PTR);
                Ok(handle)
            }
        }
    }

    pub fn set_title(mut self, name: String) -> Self {
        self.window_name = name;
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

pub struct Window {
    handle:     HWND,
    controls:   Vec<Box<Paintable>>,
}

impl Window {
    pub fn from_handle(handle: HWND) -> Self {
        Window {
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

impl Paintable for Window {
    fn paint(&self, context: &PaintContext) {
        for c in self.controls.iter().filter(|c| c.needs_repaint(context)) {
            c.paint(context);
        }
    }
}
