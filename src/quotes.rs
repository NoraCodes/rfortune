use std::fs::File;
use std::path::PathBuf;
use csv;
use rand;
use database;

use rocket::config;
use rocket::Error;

pub type Quote = (String, String, Option<String>);

pub fn get_source_from_quote_as_text(quote: &Quote) -> String {
    match quote.2 {
        Some(ref s) => s.clone(),
        None => String::from("No source.")
    }
}

pub fn get_source_from_quote_as_json(quote: &Quote) -> String {
    match quote.2 {
        Some(ref s) => format!("\"{}\"", s),
        None => String::from("null")
    }
}

pub fn get_random_quote() -> Option<Quote> {
    let current_config = match config::active() {
        Some(c) => c,
        None => {return None;}
    };
    let db_path: String = match current_config.get_str("db_path") {
        Ok(v) => v.into(),
        Err(_) => {return None;}
    };
    let mut connection = match database::get_database_connection(db_path) {
        Ok(c) => c,
        Err(_) => {return None;}
    };

    let quotes_list = match database::get_quotes(&mut connection) {
        Ok(q) => q,
        Err(_) => {return None;}
    };
    let random_number = rand::random::<usize>() % quotes_list.len();
    Some(quotes_list[random_number].clone())
}
