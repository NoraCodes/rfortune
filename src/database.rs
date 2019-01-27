use crate::quotes::Quote;
use rusqlite::{Connection, Error};

macro_rules! to_sql {
    () => {
        $crate::NO_PARAMS
    };
    ($($param:expr),+ $(,)*) => {
        &[$(&$param as &dyn rusqlite::types::ToSql),+]
    };
}

// SQL to create the quotes table in the database
const SQL_INIT_DATABASE: &'static str = " CREATE TABLE quotes (
    id          INTEGER PRIMARY KEY,
    quote       TEXT NOT NULL,
    author      VARCHAR(255),
    source      TEXT
)";

const SQL_INSERT_QUOTE: &'static str =
    " INSERT INTO quotes (quote, author, source) VALUES (?1, ?2, ?3)";

const SQL_QUERY_ALL_QUOTES: &'static str = " SELECT * FROM quotes ";

const SQL_QUERY_RANDOM_QUOTE: &'static str =
    " SELECT * FROM quotes WHERE id IN (SELECT id FROM quotes ORDER BY RANDOM() LIMIT 1) ";

pub fn get_database_connection(location: String) -> Result<Connection, Error> {
    Connection::open(location)
}

pub fn initialize(connection: &mut Connection) -> Result<(), Error> {
    connection.execute(SQL_INIT_DATABASE, &[])?;
    Ok(())
}

pub fn add_quote(connection: &mut Connection, quote: &Quote) -> Result<(), Error> {
    let mut statement = connection.prepare(SQL_INSERT_QUOTE)?;
    statement.execute(to_sql![quote.quote, quote.author, quote.source])?;
    Ok(())
}

pub fn get_quotes(connection: &mut Connection) -> Result<Vec<Quote>, Error> {
    let mut statement = connection.prepare(SQL_QUERY_ALL_QUOTES)?;
    let maybe_quotes_iter = statement.query_map(&[], |row| {
        Quote::new(
            row.get::<_, String>(1),
            row.get::<_, String>(2),
            row.get::<_, Option<String>>(3),
        )
    })?;

    let mut quotes = Vec::new();
    for quote in maybe_quotes_iter {
        quotes.push(quote.unwrap());
    }

    return Ok(quotes);
}

pub fn get_random_quote(connection: &mut Connection) -> Result<Option<Quote>, Error> {
    let mut statement = connection.prepare(SQL_QUERY_RANDOM_QUOTE)?;
    let maybe_quote = statement
        .query_map(&[], |row| {
            Quote::new(
                row.get::<_, String>(1),
                row.get::<_, String>(2),
                row.get::<_, Option<String>>(3),
            )
        })?
        .next();
    match maybe_quote {
        None => Ok(None),
        Some(r) => Ok(r.ok()),
    }
}
