# rfortune
A simple Rust `fortune`-like webapp with Rocket

![A screenshot of rfortune in action](https://github.com/SilverWingedSeraph/rfortune/blob/master/screenshot.png)

## What
This simple webapp reads quotes from a SQLite database and picks a random one to return
with each request. It uses templates for HTML responses and `serde` for JSON.
It provides a proof of concept for my use of Rocket.

What this application demonstrates:
- [x] Database with `rusqlite`
- [x] Templates with `tera`
- [x] JSON API with `serde` and the `rocket_contrib` JSON support
- [x] Integrated command line management
- [x] Form data entry
- [x] Responsive, mobile-first design
- [ ] Custom error pages
- [ ] Rocket tests

## Why
I like Rocket, as a concept. It allows me to use Rust, a language that tends to
produce code which is both fast and correct, as a backend for services which
typically need to be both fast and correct - web services.

Rocket also works with paradigms I'm familiar with: pluggable datastore and
template modules connected by route functions, like Flask.

## How
To run the application, simply clone the repo and, assuming that you have
a nightly Rust toolchain installed, as well as `libsqlite3-dev`, run
`cargo run` in the root directory - you'll get a webserver running!

You'll need to add quotes using either `cargo run add dev.db <quote> <author> [source]`
or the JSON API.

## License
This app is licensed under the Unlicense. Use it for whatever you want; if
it breaks something it's not my fault.
