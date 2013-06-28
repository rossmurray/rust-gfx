#initial commit. does not compile.

# rust-gfx
Bindings for SDL_gfx in Rust

# Overview

rust-gfx is a rust library that wraps the functions in SDL_gfx to make them easy to use in Rust. SDL_gfx is a C library for drawing primitives like circles, squares and lines to an SDL_Surface. This library depends on Rust-SDL.

Rust-SDL uses the CC0 license.

# Requirements

* *Rust* - compiled against *Master*.
* *SDL 1.2 development libraries* - recommended you install through package management. Otherwise: http://www.libsdl.org/
* *SDL_gfx 1.2 development library* - recommended you install through package management. Otherwise: http://www.ferzkopp.net/joomla/content/view/19/14/
* *Rust-SDL* - 

# Compiling
Once you have Rust-SDL compiled, compiling rust-gfx is simply:

> rustc -L path/to/Rust-SDL gfx.rc

# Demo
Coming