# raytracer
A simple raytracer written in Rust, following [raytracing.github.io](https://raytracing.github.io)

This expands on the book in several ways, including multithreading, multiple scenes, and multiframe animations.
Rendering takes a while. 
This doesn't leverage the GPU, so if you're using the complex scene at a high resolution and sample rate, be prepared to wait ~10 minutes on a decent CPU.
