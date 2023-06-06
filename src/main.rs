#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rusqlite;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod args;
mod database;
mod quotes;
mod routes;

use rocket::fairing::AdHoc;
use std::sync::RwLock;

lazy_static! {
    pub static ref BASE_URL: RwLock<Option<String>> = RwLock::new(None);
    pub static ref CSS_URLS: RwLock<Option<Vec<String>>> = RwLock::new(None);
}

#[database("sqlite")]
pub struct SqliteDb(rusqlite::Connection);

impl SqliteDb {
    pub fn connection(&mut self) -> &mut rusqlite::Connection {
        &mut self.0
    }
}

use args::Mode;

fn main() {
    let rc = fake_main();
    std::process::exit(rc);
}

fn fake_main() -> i32 {
    let given_arguments: Vec<_> = std::env::args().collect();
    let parsing_results = match args::parse_args(&given_arguments) {
        Ok(v) => v,
        Err(s) => {
            println!("{}", s);
            return 1;
        }
    };
    let mode: args::Mode = parsing_results.0;
    let database_path: String = parsing_results.1;
    let quote_to_add: Option<quotes::Quote> = parsing_results.2;

    println!("Opening SQLite database at {:?}.", database_path);
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
            rocket::ignite()
                .mount(
                    "/",
                    routes![
                        routes::index_html,
                        routes::all,
                        routes::add_form,
                        routes::add,
                        routes::api_html,
                        routes::json,
                        routes::json_all,
                        routes::json_add
                    ],
                )
                .register(catchers![routes::error_404])
                .attach(SqliteDb::fairing())
                .attach(rocket_contrib::templates::Template::fairing())
                .attach(AdHoc::on_attach("Base URL", |rocket| {
                    println!("Adding baseurl managed state from config...");
                    let base_url_config = rocket.config().get_str("base_url");
                    let css_url_config = rocket.config().get_slice("css_urls");
                    {
                        let mut base_handle = BASE_URL.write().expect("BASE_URL poisoned");
                        *base_handle = base_url_config.ok().map(|s| s.to_owned());
                    }
                    {
                        let mut css_handle = CSS_URLS.write().expect("CSS_URLS poisoned");
                        *css_handle = css_url_config.ok().map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                                .collect()
                        });
                    }
                    Ok(rocket)
                }))
                .launch();
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
                let source_text = quote.get_source_as_text();
                println!(
                    "Quote: {} - {} ({})",
                    quote.quote, quote.author, source_text
                );
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
    return 0;
}
