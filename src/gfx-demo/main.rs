extern mod sdl;
extern mod gfx;

use sdl::video;
use sdl::wm;
use sdl::event;

use std::rand;
use std::io::timer;

use std::libc::{c_int};
extern {
	pub fn usleep(n : c_int) -> c_int;
}

fn main() {
	sdl::init([sdl::InitEverything]);
	wm::set_caption("rust-gfx test", "rust-gfx");
	let SWIDTH: i16 = 1200;
	let SHEIGHT: i16 = 700;
	let screen = match sdl::video::set_video_mode(SWIDTH as int, SHEIGHT as int, 32, 
			[sdl::video::HWSurface], [sdl::video::DoubleBuf, video::Fullscreen]) {
		Ok(screen) => screen,	
		Err(err) => fail!(format!("failed to set video mode: {}", err))
	};

	let bg = video::RGB(0, 0, 0);
	screen.fill(bg);
	loop {
		if rand::random() {
			for _ in range(0, 5) {
				gfx::filled_circle(screen, rand(10, SWIDTH - 10), rand(10, SHEIGHT - 10), rand(4, 45), 
					&rand::random::<sdl::video::Color>());
			}
		}
		else {
			for _ in range(0, 5) {
				let x = rand(10, SWIDTH - 10);
				let y = rand(10, SHEIGHT - 10);
				let x2 = x + rand(3, 60);
				let y2 = y + rand(3, 60);
				gfx::filled_rect(screen, x, y, x2, y2, &rand::random::<sdl::video::Color>());
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
	let mut r = rand::random::<u16>() as i16;
	r = r % diff;
	r = r + min;
	r
}

fn check_exit_input() -> bool {
	loop {
		match event::poll_event() {
			event::QuitEvent => return true,
			event::KeyEvent(event::EscapeKey, true , _, _) => {
				return true;
			},
			event::NoEvent => return false,
			_ => ()
		};
	}
}