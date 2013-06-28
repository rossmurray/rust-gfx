use std::libc::{uint8_t, int16_t, int32_t};

use sdl::video::ll::SDL_Surface;
use sdl::video::Surface;
use sdl::video::Color;
use sdl::video::RGB;
use sdl::video::RGBA;

#[link_args="-lSDL_gfx"]
extern {
	fn rectangleRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t,
		r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> int32_t;

	fn boxRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t,
		r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> int32_t;

	fn filledCircleRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t,
		r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> int32_t;
}

pub fn rectangle_rgba(screen: &Surface, x1: i16, y1: i16, x2: i16, y2: i16, r: u8, g: u8, b: u8, a: u8) {
	unsafe {
		rectangleRGBA(screen.raw, x1, y1, x2, y2, r, g, b, a);
	}
}

pub fn box_rgba(screen: &Surface, x1: i16, y1: i16, x2: i16, y2: i16, r: u8, g: u8, b: u8, a: u8) {
	unsafe {
		boxRGBA(screen.raw, x1, y1, x2, y2, r, g, b, a);
	}
}

pub fn filled_circle(screen: &Surface, x: i16, y: i16, radius: i16, color: &Color) {
	unsafe {
		match *color {
			RGB(r, g, b) => filledCircleRGBA(screen.raw, x, y, radius, r, g, b, 255),
			RGBA(r, g, b, a) => filledCircleRGBA(screen.raw, x, y, radius, r, g, b, a),
		};
	}
}