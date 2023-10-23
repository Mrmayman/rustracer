# Grinding Metal
A real time raytraced racing game in Rust.

Currently I am working on the software raytracer engine. Once I'm done, I'll focus on the game.
The engine is a direct port of the "Ray Tracing In One Weekend" book and its sequel to rust.

The `src_old` folder contains the old single threaded version of the engine.
The `src` folder contains a much faster multi threaded rewrite of the engine, which uses 12 threads by default.

# Features:
## New version
- Everything from the first book, except for Dielectric material and depth of field focus blur.

## Old version
- Everything from the first and second book except for noise textures.s

# Build
- Install Rust
- Clone the repo `git clone https://github.com/Mrmayman/grinding-metal.git`
- Navigate inside the new grinding-metal folder `cd grinding-metal`
- Build and run it `cargo run --release`

## Credits
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
