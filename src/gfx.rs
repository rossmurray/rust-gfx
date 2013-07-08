#[link(name = "gfx",
       vers = "0.0.1",
       uuid = "9341ba57-6ef9-4a76-a6d9-f3ec9a19d9f4",
       url = "http://github.com/rossmurray/rust-gfx")];

#[comment = "SDL_gfx bindings"];
#[license = "CC0"];
#[crate_type = "lib"];

extern mod sdl;

use std::libc::{c_int, c_schar, uint8_t, int16_t, uint32_t};

use sdl::video::ll::SDL_Surface;
use sdl::video::Surface;
use sdl::video::Color;
use sdl::video::RGB;
use sdl::video::RGBA;

#[link_args="-lSDL_gfx"]
extern {
	// fn rectangleRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t,
	// 	r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	// fn boxRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t,
	// 	r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	// fn filledCircleRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t,
	// 	r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn pixelColor(dst: *SDL_Surface, x: int16_t, y: int16_t, color: uint32_t) -> c_int;

	fn pixelRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn hlineColor(dst: *SDL_Surface, x1: int16_t, x2: int16_t, y: int16_t, color: uint32_t) -> c_int;

	fn hlineRGBA(dst: *SDL_Surface, x1: int16_t, x2: int16_t, y: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn vlineColor(dst: *SDL_Surface, x: int16_t, y1: int16_t, y2: int16_t, color: uint32_t) -> c_int;

	fn vlineRGBA(dst: *SDL_Surface, x: int16_t, y1: int16_t, y2: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn rectangleColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, color: uint32_t) -> c_int;

	fn rectangleRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn roundedRectangleColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, rad: int16_t, color: uint32_t) -> c_int;

	fn roundedRectangleRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, rad: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn boxColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, color: uint32_t) -> c_int;

	fn boxRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn roundedBoxColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, rad: int16_t, color: uint32_t) -> c_int;

	fn roundedBoxRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, rad: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn lineColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, color: uint32_t) -> c_int;

	fn lineRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn aalineColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, color: uint32_t) -> c_int;

	fn aalineRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn thickLineColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, width: uint8_t, color: uint32_t) -> c_int;

	fn thickLineRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, width: uint8_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn circleColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, color: uint32_t) -> c_int;

	fn circleRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn arcColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, start: int16_t, end: int16_t, color: uint32_t) -> c_int;

	fn arcRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, start: int16_t, end: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn aacircleColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, color: uint32_t) -> c_int;

	fn aacircleRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn filledCircleColor(dst: *SDL_Surface, x: int16_t, y: int16_t, r: int16_t, color: uint32_t) -> c_int;

	fn filledCircleRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn ellipseColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rx: int16_t, ry: int16_t, color: uint32_t) -> c_int;

	fn ellipseRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rx: int16_t, ry: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn aaellipseColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rx: int16_t, ry: int16_t, color: uint32_t) -> c_int;

	fn aaellipseRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rx: int16_t, ry: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn filledEllipseColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rx: int16_t, ry: int16_t, color: uint32_t) -> c_int;

	fn filledEllipseRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rx: int16_t, ry: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn pieColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, start: int16_t, end: int16_t, color: uint32_t) -> c_int;

	fn pieRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, start: int16_t, end: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn filledPieColor(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, start: int16_t, end: int16_t, color: uint32_t) -> c_int;

	fn filledPieRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, rad: int16_t, start: int16_t, end: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn trigonColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, x3: int16_t, y3: int16_t, color: uint32_t) -> c_int;

	fn trigonRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, x3: int16_t, y3: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn aatrigonColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, x3: int16_t, y3: int16_t, color: uint32_t) -> c_int;

	fn aatrigonRGBA(dst: *SDL_Surface,  x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, x3: int16_t, y3: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	fn filledTrigonColor(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, x3: int16_t, y3: int16_t, color: uint32_t) -> c_int;

	fn filledTrigonRGBA(dst: *SDL_Surface, x1: int16_t, y1: int16_t, x2: int16_t, y2: int16_t, x3: int16_t, y3: int16_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	//fn polygonColor(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, color: uint32_t) -> c_int;

	//fn polygonRGBA(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	//fn aapolygonColor(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, color: uint32_t) -> c_int;

	//fn aapolygonRGBA(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	//fn filledPolygonColor(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, color: uint32_t) -> c_int;

	//fn filledPolygonRGBA(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	//fn texturedPolygon(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, texture: *SDL_Surface, texture_dx: int32_t, texture_dy: int32_t) -> c_int;

	//fn filledPolygonColorMT(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, color: uint32_t, int **polyInts, int *polyAllocated) -> c_int;

	//fn filledPolygonRGBAMT(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t, int **polyInts, int *polyAllocated) -> c_int;

	//fn texturedPolygonMT(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, texture: *SDL_Surface, texture_dx: int32_t, texture_dy: int32_t, int **polyInts, int *polyAllocated) -> c_int;

	//fn bezierColor(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, s: int32_t, color: uint32_t) -> c_int;

	//fn bezierRGBA(dst: *SDL_Surface, const int16_t * vx, const int16_t * vy, n: int32_t, s: int32_t, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	//fn gfxPrimitivesSetFont(const void *fontdata, cw: uint32_t, ch: uint32_t);

	fn gfxPrimitivesSetFontRotation(rotation: uint32_t);

	fn characterColor(dst: *SDL_Surface, x: int16_t, y: int16_t, c: c_schar, color: uint32_t) -> c_int;

	fn characterRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, c: c_schar, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;

	//fn stringColor(dst: *SDL_Surface, x: int16_t, y: int16_t, const char *s, color: uint32_t) -> c_int;

	//fn stringRGBA(dst: *SDL_Surface, x: int16_t, y: int16_t, const char *s, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;
}

pub fn rectangle(screen: &Surface, x1: i16, y1: i16, x2: i16, y2: i16, color: &Color) {
	let (r,g,b,a) = map_color(screen, color);
	unsafe {
		rectangleRGBA(screen.raw, x1, y1, x2, y2, r, g, b, a);
	}
}

pub fn box(screen: &Surface, x1: i16, y1: i16, x2: i16, y2: i16, color: &Color) {
	let (r,g,b,a) = map_color(screen, color);
	unsafe {
		boxRGBA(screen.raw, x1, y1, x2, y2, r, g, b, a);
	}
}

pub fn filled_circle(screen: &Surface, x: i16, y: i16, radius: i16, color: &Color) {
	let (r,g,b,a) = map_color(screen, color);
	unsafe {
		filledCircleRGBA(screen.raw, x, y, radius, r, g, b, a);
	}
}

pub fn pixel(screen: &Surface, x: i16, y: i16, color: &Color) {
	let (r,g,b,a) = map_color(screen, color);
	unsafe {
		pixelRGBA(screen.raw, x, y, r, g, b, a);
	}
}

pub fn line(screen: &Surface, x1: i16, y1: i16, x2: i16, y2: i16, color: &Color) {
	let (r,g,b,a) = map_color(screen, color);
	unsafe {
		lineRGBA(screen.raw, x1, y1, x2, y2, r, g, b, a);
	}
}

//this is temporary.
//there's a bug with the SDL lib version of this from rust-sdl that I haven't figured out yet.
fn map_color(_screen: &Surface, color: &Color) -> (u8, u8, u8, u8) {
	match *color {
		RGB(r,g,b) => (r,g,b,255),
		RGBA(r,g,b,a) => (r,g,b,a)
	}
}