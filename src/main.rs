#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;
extern crate csv;

mod quotes;
mod routes;
mod args;

use std::path::PathBuf;

fn main() {
    let rc = fake_main();
    std::process::exit(rc);
}

fn fake_main() -> i32 {
    let given_arguments: Vec<_> = std::env::args().collect();
    let parsing_results = match args::parse_args(given_arguments) {
        Ok(v) => v,
        Err(s) => {println!("{}", s); return 1;}
    };
    let do_init: bool = parsing_results.0;
    let database_path: String = parsing_results.1;

    if do_init {
        if database_path == ":memory:" {
            println!("Initializing SQLite database in memory.");
        } else {
            println!("Initializing SQLite database at {:?}.", database_path);
        }
    } else {
        rocket::ignite().mount("/", routes![routes::index_html, routes::json]).launch();
    }
    return 0
}
