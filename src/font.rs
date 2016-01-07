use winapi::*;
use gdi32::*;
use kernel32::*;
use wio::wide::*;

use std::ops::Deref;

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
            // Seems to be missing on most version of windows.
            ClipPrecision::DFAOverride      => unimplemented!(),
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
            let mut face_wide = self.face.to_wide_null();
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

    pub fn set_face<T: Into<String>>(&mut self, face: T) -> &mut Self {
        self.face = face.into();
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
