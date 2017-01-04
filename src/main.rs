#![feature(plugin)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(attr_literals)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;
extern crate csv;

mod quotes;
mod routes;

use std::path::PathBuf;

fn main() {
    rocket::ignite().mount("/", routes![routes::index_html, routes::json]).launch();
}
