#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;
extern crate csv;
extern crate rusqlite;

mod quotes;
mod routes;
mod args;
mod database;

use args::Mode;

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
    let mode: args::Mode = parsing_results.0;
    let database_path: String = parsing_results.1;
    let quote_to_add: Option<quotes::Quote> = parsing_results.2;

    if database_path == ":memory:" {
    println!("Opening SQLite database in memory.");
    } else {
        println!("Opening SQLite database at {:?}.", database_path);
    }
    let mut db_connection = database::get_database_connection(database_path).unwrap();

    match mode {
        Mode::Initialize => {
            let res = database::initialize(&mut db_connection);
            if res.is_err() {
                    println!("[FATAL] {:?}", res);
                    return 1;
            }
            println!("Initialized SQLite Database.");
        }
        Mode::Execute => {
            rocket::ignite().mount("/", routes![routes::index_html, routes::json]).launch();
        }
        Mode::List => {
            let maybe_quotes = database::get_quotes(&mut db_connection);
            let quotes_vec = match maybe_quotes {
                Ok(q) => q,
                Err(e) => {
                    println!("[FATAL] {:?}", e);
                    return 1;
                }
            };
            for quote in quotes_vec {
                let source = quotes::get_source_from_quote_as_text(&quote);
                println!("Quote: {} - {} ({})", quote.0, quote.1, source);
            }
        }
        Mode::Add => {
            if quote_to_add.is_none() {
                println!("[FATAL] Asked to add a nonexistant quote.");
                return 1;
            }
            let result = database::add_quote(&mut db_connection, &quote_to_add.unwrap());
            if result.is_err() {
                println!("[FATAL] Failed to add quote. {:?}", result.err().unwrap())
            }
        }
    };
    return 0
}
