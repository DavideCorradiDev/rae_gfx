use std::default::Default;

pub use wgpu::Color as ColorF64;

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
}

impl Default for Color {
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

impl as_slice::AsSlice for Color {
    type Element = u8;
    fn as_slice(&self) -> &[Self::Element] {
        let pc: *const Color = self;
        let pc: *const u8 = pc as *const u8;
        unsafe { std::slice::from_raw_parts(pc, std::mem::size_of::<Color>()) }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ColorF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorF32 {
    pub const TRANSPARENT: Self = Self {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    pub const BLACK: Self = Self {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
    pub const WHITE: Self = Self {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };
    pub const RED: Self = Self {
        r: 1.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
    pub const GREEN: Self = Self {
        r: 0.,
        g: 1.,
        b: 0.,
        a: 1.,
    };
    pub const BLUE: Self = Self {
        r: 0.,
        g: 0.,
        b: 1.,
        a: 1.,
    };
    pub const YELLOW: Self = Self {
        r: 1.,
        g: 1.,
        b: 0.,
        a: 1.,
    };
    pub const CYAN: Self = Self {
        r: 0.,
        g: 1.,
        b: 1.,
        a: 1.,
    };
    pub const MAGENTA: Self = Self {
        r: 1.,
        g: 0.,
        b: 1.,
        a: 1.,
    };
}

impl Default for ColorF32 {
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

impl as_slice::AsSlice for ColorF32 {
    type Element = f32;
    fn as_slice(&self) -> &[Self::Element] {
        let pc: *const ColorF32 = self;
        let pc: *const u8 = pc as *const u8;
        let data = unsafe { std::slice::from_raw_parts(pc, std::mem::size_of::<ColorF32>()) };
        bytemuck::cast_slice(&data)
    }
}

impl From<ColorF64> for Color {
    fn from(c: ColorF64) -> Self {
        const FACTOR: f64 = 255.;
        let r = num::clamp(c.r * FACTOR, 0., 255.) as u8;
        let g = num::clamp(c.g * FACTOR, 0., 255.) as u8;
        let b = num::clamp(c.b * FACTOR, 0., 255.) as u8;
        let a = num::clamp(c.a * FACTOR, 0., 255.) as u8;
        Self { r, g, b, a }
    }
}

impl From<Color> for ColorF64 {
    fn from(c: Color) -> Self {
        const FACTOR: f64 = 255.;
        let r = c.r as f64 / FACTOR;
        let g = c.g as f64 / FACTOR;
        let b = c.b as f64 / FACTOR;
        let a = c.a as f64 / FACTOR;
        Self { r, g, b, a }
    }
}

impl From<ColorF32> for Color {
    fn from(c: ColorF32) -> Self {
        const FACTOR: f32 = 255.;
        let r = num::clamp(c.r * FACTOR, 0., 255.) as u8;
        let g = num::clamp(c.g * FACTOR, 0., 255.) as u8;
        let b = num::clamp(c.b * FACTOR, 0., 255.) as u8;
        let a = num::clamp(c.a * FACTOR, 0., 255.) as u8;
        Self { r, g, b, a }
    }
}

impl From<Color> for ColorF32 {
    fn from(c: Color) -> Self {
        const FACTOR: f32 = 255.;
        let r = c.r as f32 / FACTOR;
        let g = c.g as f32 / FACTOR;
        let b = c.b as f32 / FACTOR;
        let a = c.a as f32 / FACTOR;
        Self { r, g, b, a }
    }
}
