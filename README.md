# What this is

This is the code for the book ray tracing in one weekend with some minor changes ported to Rust (not C++).

Essentially it is a basic ray tracer capable of rendering simple spheres in 3 different materials: lambertian, metal and dielectric (glass).

It was made for fun (and to get a better understanding of Rust and raytracers).

# Running it
`cargo run 100` pipes the ppm data to stdout (and progress indicator to stderr).

On a linux box with feh installed you can test it like this: `cargo run 100  > test.ppm && feh --zoom fill  test.ppm`

# Example image
![Example image](https://github.com/freekh/rust-ray-trace-one-weekend/raw/master/image.png)
