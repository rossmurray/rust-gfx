extern mod sdl;
extern mod gfx;

use std::i16;
use sdl::video;
use sdl::wm;
use sdl::event;

use std::rand::RngUtil;
use std::rand;

use std::libc::{c_int};
pub extern {
	fn usleep(n : c_int) -> c_int;
}

fn main() {
	do sdl::start {
		sdl_main();
	}
}

fn sdl_main() {
	sdl::init([sdl::InitEverything]);
	wm::set_caption("rust-gfx test", "rust-gfx");
	let SWIDTH: i16 = 1200;
	let SHEIGHT: i16 = 800;
	let screen = match sdl::video::set_video_mode(SWIDTH as int, SHEIGHT as int, 32, [sdl::video::HWSurface], [sdl::video::DoubleBuf]) {
		Ok(screen) => screen,
		Err(err) => fail!(fmt!("failed to set video mode: %s", err))
	};
	let mut rng = rand::rng();

	let bg = video::RGB(0, 0, 0);
	screen.fill(bg);
	loop {
		if rng.gen() {
			for i16::range(0, 5) |_i| {
				gfx::filled_circle(screen, rand(10, SWIDTH - 10), rand(10, SHEIGHT - 10), rand(4, 45), &rng.gen::<sdl::video::Color>());
			}
		}
		else {
			for i16::range(0, 5) |_i| {
				let x = rand(10, SWIDTH - 10);
				let y = rand(10, SHEIGHT - 10);
				let x2 = x + rand(3, 60);
				let y2 = y + rand(3, 60);
				gfx::box(screen, x, y, x2, y2, &rng.gen::<sdl::video::Color>());
			}
		}
		if check_exit_input() == true {
			break;
		}
		screen.flip();
		unsafe {
			usleep(20 * 1000);
		}
	}
	sdl::quit();
}

fn rand(min: i16, max: i16) -> i16 {
	let diff = max - min;
	let mut r = rand::random::<i16>();
	if r < 0 {
		r = r * -1;
	}
	r = r % diff;
	r = r + min;
	r
}

fn wait_for_exit() {
	loop {
		match event::wait_event() {
			event::KeyEvent(key, true , _, _) => {
				match (key) {
					event::EscapeKey => return,
					_ => ()
				}
			},
			_ => ()
		};
	}
}

fn check_exit_input() -> bool {
	loop {
		match event::poll_event() {
			event::KeyEvent(key, true , _, _) => {
				match (key) {
					event::EscapeKey => return true,
					_ => ()
				}
			},
			event::NoEvent => return false,
			_ => ()
		};
	}
}