use crate::database;
use rusqlite::Connection;

#[derive(Clone, Serialize, Deserialize, FromForm)]
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

pub fn get_random_quote(connection: &mut Connection) -> Option<Quote> {
    match database::get_random_quote(connection) {
        Ok(q) => q,
        Err(_) => {return None;}
    } 
}

pub fn get_quotes(connection: &mut Connection) -> Option<Vec<Quote>> {
    match database::get_quotes(connection) {
        Ok(q) => Some(q),
        Err(_) => None
    }
}

pub fn add_quote(quote: &Quote, connection: &mut Connection) -> Option<()> {
    match database::add_quote(connection, quote) {
        Ok(_) => Some(()),
        Err(_) => None
    }
}

