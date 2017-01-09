use rocket::response::content::{JSON};
use rocket::request::Form;
use rocket_contrib;
use rocket_contrib::Template;
use serde_json;
use quotes;
use quotes::Quote;

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

#[derive(Serialize)]
struct QuoteListTemplateContext {
    quotes: Vec<QuoteTemplateContext>
}

impl QuoteListTemplateContext {
    fn new(quotes: Vec<QuoteTemplateContext>) -> QuoteListTemplateContext {
        QuoteListTemplateContext {
            quotes: quotes
        }
    }
}

#[derive(Serialize)]
struct AddFormContext {
    message: String
}

#[get("/")]
pub fn index_html() -> Template {
    let quote = quotes::get_random_quote().unwrap();
    let source_text = quote.get_source_as_text();
    let context = QuoteTemplateContext::new(quote.quote, quote.author, source_text);
    Template::render("index", &context)
}

#[get("/all")]
pub fn all() -> Template {
    let quotes = quotes::get_quotes().unwrap();
    let mut contexts = Vec::with_capacity(quotes.len());
    for quote in quotes {
        let source_text = quote.get_source_as_text();
        contexts.push(QuoteTemplateContext::new(quote.quote, quote.author, source_text));
    }
    Template::render("list", &QuoteListTemplateContext::new(contexts))
}

#[get("/add")]
pub fn add_form() -> Template {
    Template::render("add", &AddFormContext{message:"".into()})
}

#[post("/add", data="<quote_data>")]
pub fn add(quote_data: Form<Quote>) -> Template {
    let mut quote: Quote = quote_data.get().clone();
    if quote.quote == "" {
        return Template::render("add", &AddFormContext{message:"Quote must have text.".into()});
    }
    if quote.author == "" {
        return Template::render("add", &AddFormContext{message:"Quote must have an author.".into()});
    }
    if quote.source == Some("".into()) {
        quote.source = None;
    }
    match quotes::add_quote(&quote) {
        Some(_) => Template::render("add", &AddFormContext{message:"Successfully added quote.".into()}),
        None => Template::render("add", &AddFormContext{message:"Failed to add quote.".into()})
    }
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

#[post("/json/add", format="application/json", data="<quote>")]
pub fn json_add(quote: rocket_contrib::JSON<Quote>) -> JSON<String> {
    match quotes::add_quote(&quote.0) {
        Some(_) => JSON(serde_json::to_string(&true).unwrap()),
        None => JSON(serde_json::to_string(&false).unwrap())
    }
}
