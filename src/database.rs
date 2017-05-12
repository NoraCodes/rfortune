use rusqlite::{Connection, Error};
use quotes::Quote;

// SQL to create the quotes table in the database
const SQL_INIT_DATABASE: &'static str =
" CREATE TABLE quotes (
    id          INTEGER PRIMARY KEY,
    quote       TEXT NOT NULL,
    author      VARCHAR(255),
    source      TEXT
)";


const SQL_INSERT_QUOTE: &'static str =
" INSERT INTO quotes (quote, author, source) VALUES (?1, ?2, ?3)";

const SQL_QUERY_ALL_QUOTES: &'static str =
" SELECT * FROM quotes ";

pub fn get_database_connection(location: String) -> Result<Connection, Error> {
    Connection::open(location)
}

pub fn initialize(connection: &mut Connection) -> Result<(), Error> {
    connection.execute(SQL_INIT_DATABASE, &[])?;
    Ok(())
}

pub fn add_quote(connection: &mut Connection, quote: &Quote) -> Result<(), Error> {
    connection.execute(SQL_INSERT_QUOTE, &[&quote.quote, &quote.author, &quote.source])?;
    Ok(())
}

pub fn get_quotes(connection: &mut Connection) -> Result<Vec<Quote>, Error> {
    let mut statement = connection.prepare(SQL_QUERY_ALL_QUOTES)?;
    let maybe_quotes_iter = statement.query_map(&[], |row| {
        Quote::new(row.get::<_, String>(1), row.get::<_, String>(2), row.get::<_, Option<String>>(3))
    })?;

    let mut quotes = Vec::new();
    for quote in maybe_quotes_iter {
        quotes.push(quote.unwrap());
    }

    return Ok(quotes)
}
