# rfortune
A simple Rust fortune-like webapp with Rocket

![A screenshot of rfortune in action](screenshot.jpg)

## What
This simple webapp reads quotes from a CSV file and picks a random one to return
with each request. It provides hard-coded HTML and JSON responses and is
generally not a very "elegant" application, but it provides a proof of concept
for my use of Rocket.

# How
To run the application, simply download the app and, assuming that you have
a nightly Rust toolchain installed, run `cargo run` in the root directory.
