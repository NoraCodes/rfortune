use rand;
use database;

use rocket::config;

#[derive(Clone, Serialize, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub source: Option<String>
}

impl Quote {
    pub fn new(quote: String, author: String, source: Option<String>) -> Quote {
        Quote {
            quote: quote,
            author: author,
            source: source
        }
    }
    pub fn get_source_as_text(self: &Self) -> String {
        match self.source {
            Some(ref s) => s.clone(),
            None => String::from("No source.")
        }
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
