use std::fs::File;
use std::path::PathBuf;
use csv;
use rand;

pub type Quote = (String, String, Option<String>);

pub fn get_source_text_from_quote(quote: &Quote) -> String {
    match quote.2 {
        Some(ref s) => s.clone(),
        None => String::from("No source.")
    }
}

pub fn get_source_json_from_quote(quote: &Quote) -> String {
    match quote.2 {
        Some(ref s) => format!("\"{}\"", s),
        None => String::from("null")
    }
}

pub fn get_random_quote(filename: &PathBuf) -> Result<Quote, csv::Error> {
    let quotes_list = get_quotes_from_file(filename)?;
    let random_number = rand::random::<usize>() % quotes_list.len();
    Ok(quotes_list[random_number].clone())
}

pub fn get_quotes_from_file(filename: &PathBuf) -> Result<Vec<Quote>, csv::Error> {
    let mut reader = get_quotes_reader_for_file(filename)?;
    let rows = reader.decode().collect::<csv::Result<Vec<Quote>>>()?;
    Ok(rows)
}

fn get_quotes_reader_for_file(filename: &PathBuf) -> Result<csv::Reader<File>, csv::Error> {
    let reader = csv::Reader::from_file(filename)?.has_headers(true).flexible(true);
    Ok(reader)
}
