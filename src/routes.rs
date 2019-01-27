use crate::quotes::{self, Quote};
use crate::SqliteDb;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_json;

#[derive(Serialize)]
struct QuoteTemplateContext {
    quote: String,
    author: String,
    source_text: String,
}

impl QuoteTemplateContext {
    fn new(quote: String, author: String, source_text: String) -> QuoteTemplateContext {
        QuoteTemplateContext {
            quote: quote,
            author: author,
            source_text: source_text,
        }
    }
}

#[derive(Serialize)]
struct QuoteListTemplateContext {
    quotes: Vec<QuoteTemplateContext>,
}

impl QuoteListTemplateContext {
    fn new(quotes: Vec<QuoteTemplateContext>) -> QuoteListTemplateContext {
        QuoteListTemplateContext { quotes: quotes }
    }
}

#[derive(Serialize)]
struct MessageContext {
    message: String,
}

#[get("/")]
pub fn index_html(mut db: SqliteDb) -> Template {
    let context = match quotes::get_random_quote(db.connection()) {
        Some(quote) => {
            let source_text = quote.get_source_as_text();
            QuoteTemplateContext::new(quote.quote, quote.author, source_text)
        }
        None => QuoteTemplateContext::new(
            "There are no quotes in the database.".into(),
            "".into(),
            "".into(),
        ),
    };
    Template::render("index", &context)
}

#[get("/all")]
pub fn all(mut db: SqliteDb) -> Template {
    let quotes = quotes::get_quotes(db.connection()).unwrap();
    let mut contexts = Vec::with_capacity(quotes.len());
    for quote in quotes {
        let source_text = quote.get_source_as_text();
        contexts.push(QuoteTemplateContext::new(
            quote.quote,
            quote.author,
            source_text,
        ));
    }
    Template::render("list", &QuoteListTemplateContext::new(contexts))
}

#[get("/add")]
pub fn add_form() -> Template {
    Template::render("add", &MessageContext { message: "".into() })
}

#[post("/add", data = "<quote_data>")]
pub fn add(quote_data: Form<Quote>, mut db: SqliteDb) -> Template {
    let mut quote: Quote = quote_data.clone();
    if quote.quote == "" {
        return Template::render(
            "add",
            &MessageContext {
                message: "Quote must have text.".into(),
            },
        );
    }
    if quote.author == "" {
        return Template::render(
            "add",
            &MessageContext {
                message: "Quote must have an author.".into(),
            },
        );
    }
    if quote.source == Some("".into()) {
        quote.source = None;
    }
    match quotes::add_quote(&quote, db.connection()) {
        Some(_) => Template::render(
            "add",
            &MessageContext {
                message: "Successfully added quote.".into(),
            },
        ),
        None => Template::render(
            "add",
            &MessageContext {
                message: "Failed to add quote.".into(),
            },
        ),
    }
}

#[get("/api")]
pub fn api_html() -> Template {
    Template::render("api", &MessageContext { message: "".into() })
}

#[catch(404)]
pub fn error_404() -> Template {
    Template::render("404", &MessageContext { message: "".into() })
}

#[get("/json")]
pub fn json(mut db: SqliteDb) -> Json<String> {
    let quote = quotes::get_random_quote(db.connection()).unwrap();
    Json(serde_json::to_string(&quote).unwrap())
}

#[get("/json/all")]
pub fn json_all(mut db: SqliteDb) -> Json<String> {
    let quotes = quotes::get_quotes(db.connection()).unwrap();
    Json(serde_json::to_string(&quotes).unwrap())
}

#[post("/json/add", format = "application/json", data = "<quote>")]
pub fn json_add(quote: Json<Quote>, mut db: SqliteDb) -> Json<bool> {
    match quotes::add_quote(&quote.0, db.connection()) {
        Some(_) => Json(true),
        None => Json(false),
    }
}
