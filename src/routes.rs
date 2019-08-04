use crate::quotes::{self, Quote};
use crate::{SqliteDb, BASE_URL};
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_json;

fn base_url() -> Option<String> {
    BASE_URL.read().expect("BASE_URL poisoned").clone()
}

#[derive(Serialize)]
struct QuoteTemplateContext {
    quote: String,
    author: String,
    source_text: String,
    base: Option<String>,
}

impl QuoteTemplateContext {
    fn new(
        quote: String,
        author: String,
        source_text: String,
        base: Option<String>,
    ) -> QuoteTemplateContext {
        QuoteTemplateContext {
            quote,
            author,
            source_text,
            base,
        }
    }
}

#[derive(Serialize)]
struct QuoteListTemplateContext {
    quotes: Vec<QuoteTemplateContext>,
    base: Option<String>,
}

impl QuoteListTemplateContext {
    fn new(quotes: Vec<QuoteTemplateContext>, base: Option<String>) -> QuoteListTemplateContext {
        QuoteListTemplateContext { quotes, base }
    }
}

#[derive(Serialize)]
struct MessageContext {
    message: String,
    base: Option<String>,
}

#[get("/")]
pub fn index_html(mut db: SqliteDb) -> Template {
    let context = match quotes::get_random_quote(db.connection()) {
        Some(quote) => {
            let source_text = quote.get_source_as_text();
            QuoteTemplateContext::new(quote.quote, quote.author, source_text, base_url())
        }
        None => QuoteTemplateContext::new(
            "There are no quotes in the database.".into(),
            "N/A".into(),
            "N/A".into(),
            base_url(),
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
            None,
        ));
    }
    Template::render("list", &QuoteListTemplateContext::new(contexts, base_url()))
}

#[get("/add")]
pub fn add_form() -> Template {
    Template::render(
        "add",
        &MessageContext {
            message: "".into(),
            base: base_url(),
        },
    )
}

#[post("/add", data = "<quote_data>")]
pub fn add(quote_data: Form<Quote>, mut db: SqliteDb) -> Template {
    let mut quote: Quote = quote_data.clone();
    if quote.quote == "" {
        return Template::render(
            "add",
            &MessageContext {
                base: base_url(),
                message: "Quote must have text.".into(),
            },
        );
    }
    if quote.author == "" {
        return Template::render(
            "add",
            &MessageContext {
                base: base_url(),
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
                base: base_url(),
                message: "Successfully added quote.".into(),
            },
        ),
        None => Template::render(
            "add",
            &MessageContext {
                base: base_url(),
                message: "Failed to add quote.".into(),
            },
        ),
    }
}

#[get("/api")]
pub fn api_html() -> Template {
    Template::render(
        "api",
        &MessageContext {
            base: base_url(),
            message: "".into(),
        },
    )
}

#[catch(404)]
pub fn error_404() -> Template {
    Template::render(
        "404",
        &MessageContext {
            base: base_url(),
            message: "".into(),
        },
    )
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
