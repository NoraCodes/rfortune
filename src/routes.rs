use rocket::response::content::{JSON};
use rocket_contrib::Template;
use serde_json;
use quotes;

#[derive(Serialize)]
struct QuoteTemplateContext {
    quote: String,
    author: String,
    source_text: String
}

impl QuoteTemplateContext {
    fn new(quote: String, author: String, source_text: String) -> QuoteTemplateContext {
        QuoteTemplateContext {
            quote: quote,
            author: author,
            source_text: source_text
        }
    }
}

#[get("/")]
pub fn index_html() -> Template {
    let quote = quotes::get_random_quote().unwrap();
    let source_text = quote.get_source_as_text();
    let context = QuoteTemplateContext::new(quote.quote, quote.author, source_text);
    Template::render("index", &context)
}

#[get("/json")]
pub fn json() -> JSON<String> {
    let quote = quotes::get_random_quote().unwrap();
    JSON(serde_json::to_string(&quote).unwrap())
}

#[get("/json/all")]
pub fn json_all() -> JSON<String> {
    let quotes = quotes::get_quotes().unwrap();
    JSON(serde_json::to_string(&quotes).unwrap())
}
