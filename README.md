# Grinding Metal
A real time raytraced racing game in Rust.

Currently I am working on the software raytracer engine. Once I'm done, I'll focus on the game.
The engine is a direct port of the "Ray Tracing In One Weekend" book and its sequel to rust.

# Features:
## New version
- Everything from the first book, except for Dielectric material and depth of field focus blur.
- Uses 12 threads by default.
- You can configure parameters in `main.rs` and `pixel_buffer.rs`
- Located in `src`

## Old version
- Everything from the first and second book except for noise textures.
- Located in `src_old`

# Build
- Install Rust
- Clone the repo `git clone https://github.com/Mrmayman/grinding-metal.git`
- Navigate inside the new grinding-metal folder `cd grinding-metal`
- Build and run it `cargo run --release`

## Credits
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
