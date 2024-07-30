# Ray Tracing in One Weekend, in Rust

This is a Rust implementation of the book [Ray Tracing in One
Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by
Peter Shirley. I followed this book to learn Rust and to get a better grasp of
ray tracing: don't expect the code to be idiomatic or efficient.

I used the following crates:

* [nalgebra](https://crates.io/crates/nalgebra) to use its `Vector3` type instead of 
  implementing a custom `Vec3` type.
* [indicatif](https://crates.io/crates/indicatif) for the progress bar.
* [rand](https://crates.io/crates/rand) to generate random numbers.
* [rayon](https://crates.io/crates/rayon) to parallelize the rendering.
* [console](https://crates.io/crates/console) for pretty terminal output.
