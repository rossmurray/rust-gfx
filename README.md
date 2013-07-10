# rust-gfx
Rust bindings for [SDL_gfx](http://www.ferzkopp.net/Software/SDL_gfx-2.0/Docs/html/index.html), using [Rust-SDL](https://github.com/brson/rust-sdl)

# Overview

rust-gfx is a rust library that wraps the functions in SDL_gfx to make them easy to use in Rust. SDL_gfx is a C library for drawing primitives like circles, squares and lines to an SDL_Surface. This library depends on Rust-SDL.

# Dependencies

* *Rust* - https://github.com/mozilla/rust compiled against *Master*.
* *SDL 1.2 development libraries* - install through package management or http://www.libsdl.org/
* *SDL_gfx 1.2 development library* - install through package management or: http://www.ferzkopp.net/joomla/content/view/19/14/
* *Rust-SDL* - https://github.com/brson/rust-sdl

# Compiling
Once you have Rust-SDL compiled, compiling rust-gfx is simply:

> cd rust-gfx

> rustc -L path/to/Rust-SDL/dir/ -o . src/gfx.rc

# Demo
To execute the demo:

> cd rust-gfx

> rustc -L . -L /path/to/Rust-SDL/dir/ demo/demo.rs

> ./demo/demo

#License
rust-gfx is distributed under the [CC0 license](http://creativecommons.org/publicdomain/zero/1.0/).
