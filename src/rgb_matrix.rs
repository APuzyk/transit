use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::path::Path;
use std::fmt;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct RGBMatrix {
    matrix: *mut RGBLedMatrix,
}

impl RGBMatrix {
    pub fn new(rows: i32, chained: i32, parallel: i32) -> Result<Self, &'static str> {
        if rows <= 0 || chained <= 0 || parallel <= 0 {
            return Err(RGBMatrixError::ConfigError("Invalid dimensions".to_string()))
        }
        
        let matrix = unsafe { led_matrix_create(rows, chained, parallel) };
        if matrix.is_null() {
            Err(RGBMatrixError::HardwareInitError("Failed to initialize GPIO".to_string()))
        } else {
            Ok(RGBMatrix { matrix })
        }
    }

    pub fn get_canvas(&self) -> Canvas {
        let canvas = unsafe { led_matrix_get_canvas(self.matrix) };
        Canvas { canvas }
    }
}

impl Drop for RGBMatrix {
    fn drop(&mut self) {
        unsafe { led_matrix_delete(self.matrix) }
    }
}

pub struct Canvas {
    canvas: *mut LedCanvas,
}

impl Canvas {
    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        unsafe { led_canvas_set_pixel(self.canvas, x, y, r, g, b) }
    }

    pub fn draw_text(
        &mut self,
        font: &Font,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
        text: &str,
        kerning_offset: i32,
    ) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe {
            draw_text(
                self.canvas,
                font.font,
                x,
                y,
                r,
                g,
                b,
                c_text.as_ptr(),
                kerning_offset,
            )
        }
    }

    pub fn vertical_draw_text(
        &mut self,
        font: &Font,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
        text: &str,
        kerning_offset: i32,
    ) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe {
            vertical_draw_text(
                self.canvas,
                font.font,
                x,
                y,
                r,
                g,
                b,
                c_text.as_ptr(),
                kerning_offset,
            )
        }
    }
}

pub struct Font {
    font: *mut LedFont,
}

impl Font {
    pub fn new<P: AsRef<Path>>(font_file: P) -> Result<Self, &'static str> {
        let font_path = CString::new(font_file.as_ref().to_str().unwrap()).unwrap();
        let font = unsafe { load_font(font_path.as_ptr()) };
        if font.is_null() {
            Err("Failed to load font")
        } else {
            Ok(Font { font })
        }
    }

    pub fn baseline(&self) -> i32 {
        unsafe { baseline_font(self.font) }
    }

    pub fn height(&self) -> i32 {
        unsafe { height_font(self.font) }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { delete_font(self.font) }
    }
}


#[derive(Debug)]
pub enum RGBMatrixError {
    HardwareInitError(String),
    ConfigError(String),
    // Add other error variants as needed
}

impl fmt::Display for RGBMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RGBMatrixError::HardwareInitError(msg) => write!(f, "Hardware initialization error: {}", msg),
            RGBMatrixError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl Error for RGBMatrixError {}

#[derive(Debug)]
pub enum FontError {
    FontLoadError(String),
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontError::FontLoadError(msg) => write!(f, "Font load Error: {}", msg),
        }
    }
}

impl Error for FontError {}