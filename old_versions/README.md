# Old versions
Before the current version there were a couple of failed attempts at a raytracer.
To run any of these versions, rename their folder to `src` and do `cargo run`.

## Version 1: v1
This version was a single threaded software path tracer. It computed the image on the CPU (with just a single thread) and passed the buffer to SDL2 as a texture to render it on screen.

It was REALLY slow because:
- It was running on the CPU
- It only used 1 thread (out of my 12 threads)
- It was pretty unoptimized. `dyn Object`, anyone?

## Version 2: v2
This was a rewrite of the first version to be multithreaded.

It technically works, in that it produces an ok looking image, but it seems unusually slow for a multithreaded raytracer. I don't remember why (this was back when I was a beginner to rust, so I might have made some rookie mistake).

Just like the previous version it calculates the image on the CPU and passes the buffer to SDL2 for rendering