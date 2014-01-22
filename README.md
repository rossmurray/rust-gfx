# rust-gfx

rust-gfx is a rust library for for drawing primitives like circles, squares and lines to an SDL_Surface.

It depends on [SDL_gfx](http://www.ferzkopp.net/Software/SDL_gfx-2.0/Docs/html/index.html) and [Rust-SDL](https://github.com/brson/rust-sdl).

# Dependencies

* *Rust* - https://github.com/mozilla/rust compiled against *0.10-pre (master)*.
* *SDL 1.2 development libraries* - install through package management or http://www.libsdl.org/
* *SDL_gfx 1.2 development library* - install through package management or: http://www.ferzkopp.net/joomla/content/view/19/14/
* *Rust-SDL* - https://github.com/brson/rust-sdl

# Compiling
Once you have Rust-SDL compiled, compiling rust-gfx is simply:

> cd rust-gfx

> export RUST_PATH=/path/to/rust-sdl

> rustpkg install gfx

# Demo
To execute the demo:

> cd rust-gfx

> export RUST_PATH=/path/to/rust-sdl # If not already set

> rustpkg install gfx-demo

> ./bin/gfx-demo

#License
rust-gfx is distributed under the [CC0 license](http://creativecommons.org/publicdomain/zero/1.0/).
