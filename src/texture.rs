use serde::{Deserialize, Serialize};

use crate::util::{random_between_0_1, Color, Point, Vec3};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "type")] // will expect { type: "SolidColor", ... } in JSON format
pub enum RenderableTexture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
}

impl PartialEq for RenderableTexture {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RenderableTexture::SolidColor(sc1), RenderableTexture::SolidColor(sc2)) => sc1 == sc2,
            (RenderableTexture::CheckerTexture(ct1), RenderableTexture::CheckerTexture(ct2)) => ct1 == ct2,
            _ => false,
        }
    }
}

impl Display for RenderableTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderableTexture::SolidColor(sc) => write!(f, "SolidColor: {:?}", sc),
            RenderableTexture::CheckerTexture(ct) => write!(f, "CheckerTexture: {:?}", ct),
        }
    }
}

impl Texture for RenderableTexture {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        match self {
            RenderableTexture::SolidColor(sc) => sc.value(u, v, p),
            RenderableTexture::CheckerTexture(ct) => ct.value(u, v, p),
        }
    }
}

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color;
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SolidColor {
    pub color: Color,
}

impl Display for SolidColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.color)
    }
}

impl PartialEq for SolidColor {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        self.color
    }
}

impl SolidColor {
    pub fn from_color(color: Color) -> Self {
        Self { color }
    }
    pub fn from_values(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color: Color::new(red, green, blue),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CheckerTexture {
    pub odd: SolidColor,
    pub even: SolidColor,
    inv_scale: f32,
}

impl Display for CheckerTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "odd: {:?}, even: {:?}, inv_scale: {:?}", self.odd, self.even, self.inv_scale)
    }
}

impl PartialEq for CheckerTexture {
    fn eq(&self, other: &Self) -> bool {
        self.odd == other.odd && self.even == other.even && self.inv_scale == other.inv_scale
    }
}

impl CheckerTexture {
    pub fn new(scale: f32, odd: SolidColor, even: SolidColor) -> Self {
        Self { odd, even, inv_scale: 1.0 / scale }
    }

    pub fn new_from_colors(
        scale: f32,
        color1: Color,
        color2: Color,
    ) -> Self {
        Self {
            odd: SolidColor::from_color(color1),
            even: SolidColor::from_color(color2),
            inv_scale: 1.0 / scale,
        }
    }

    pub fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
