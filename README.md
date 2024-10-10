# Rustracer
My attempt at a real-time path tracer in Rust and WGPU.

This is based on the "Ray Tracing In One Weekend" book and its sequel.

# Controls
WASD to move around. Space to go up, Left Shift to go down.

Move your mouse to look around. By default it will be locked. Click to unlock it, and click again to lock it.

# How does it work?
- It runs the path tracer in a compute shader and stores the result in a texture.
- Then it copies the texture to the screen via the fragment shader (with a fullscreen rectangle mesh).

All shaders are in `src/shaders`. The compute shader consists of `shaders/compute_shader.wgsl` and `shaders/raytracer/*`.

# Constants
If the raytracer is too slow, you can reduce the settings.

In the `src/shaders/compute_shader_wgsl` file you can find a section at the top:

```
// Tweak these settings as per your needs:
// =======================================
// Samples: Less is faster. Higher samples give less noise.
const samples = 64;
// Bounces: Less is faster. Higher bounces improve reflections.
const bounces = 4;
// Antialiasing: Fixes those jagged pixels in the edges of objects.
const antialiasing = 1;
// =======================================
```

You can edit it to match your hardware.

Additionally, in `src/main.rs` at the top there is a `SCALE_FACTOR` variable.

```
/// This indicates how much to downscale the image.
/// 4.0 means it will reduce resolution by 4x (example: from 1080p to 270p).
/// Higher values are faster but lower resolution.
const SCALE_FACTOR: f32 = 4.0;
```

This can provide a major impact to your performance.

# Build
- Install Rust
- Clone the repo `git clone https://github.com/Mrmayman/rustracer.git`
- Navigate inside the new rustracer folder `cd rustracer`
- Do `cargo run --release`

# Notes:
- If you find any problem with the code, feel free to contact me or start an issue in the repository.
- The code may not be the best, I'm still rather inexperienced in programming.
- You can check out the old (pretty horrible) versions of this in the `old_versions` folder. There's a `README.md` there too, for more information.

# Credits
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

[_Ray Tracing: The Next Week_ (I didn't implement these features yet)](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
