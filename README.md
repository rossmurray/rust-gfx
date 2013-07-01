# rust-gfx
Bindings for SDL_gfx in Rust, using [Rust-SDL](https://github.com/brson/rust-sdl)

SDL_gfx documentation: http://www.ferzkopp.net/Software/SDL_gfx-2.0/Docs/html/index.html

# Overview

rust-gfx is a rust library that wraps the functions in SDL_gfx to make them easy to use in Rust. SDL_gfx is a C library for drawing primitives like circles, squares and lines to an SDL_Surface. This library depends on Rust-SDL.

# Dependencies

* *Rust* - compiled against *Master*.
* *SDL 1.2 development libraries* - install through package management or http://www.libsdl.org/
* *SDL_gfx 1.2 development library* - install through package management or: http://www.ferzkopp.net/joomla/content/view/19/14/
* *Rust-SDL* - https://github.com/brson/rust-sdl

# Compiling
Once you have Rust-SDL compiled, compiling rust-gfx is simply:

> cd rust-gfx

> rustc -L path/to/Rust-SDL/dir/ -o . src/gfx.rc

# Demo
Soon

#License
rust-gfx is distributed under the CC0 license.
