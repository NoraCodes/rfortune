use rocket::response::content::{HTML, JSON};
use std::path::PathBuf;
use quotes;

#[get("/")]
pub fn index_html() -> HTML<String> {
    let quote = quotes::get_random_quote().unwrap();
    let source = quotes::get_source_from_quote_as_text(&quote);
    HTML(format!("
    <html>
    <head>
    <title> Rocket Fortune </title>
    <style type='text/css'>
    body {{
        background-color: black;
        color: green;
        font-family: monospace;
    }}
    a:link {{
        color: green;
        text-decoration: none;
    }}
    a:visited {{
        color: green;
        text-decoration: none;
    }}
    a:active {{
        color: green;
        text-decoration: underline;
    }}
    a:hover {{
        color: green;
        text-decoration: underline;
    }}
    h1 h2 h3 {{
        text-align: right;
    }}
    blockquote {{
        padding: 4px;
        border-radius: 2px;
        background-color: #555;
        color: white;
    }}
    .quote-container {{
        padding: 10px;
        margin: 10px;
        border-radius: 2px;
        background-color: #333;
    }}
    .main-container {{
        margin-left: 25%;
        margin-right: 25%;
        margin-top: 20px;
        padding: 10px;
        border-radius: 2px;
        min-width: 200px;
        background-color: #111;
    }}
    </style>
    </head>
    <body>
    <div class='main-container'>
    <h1>Rocket Fortune</h1>
    <h3>A simple fortune application, built with Rocket.</h3>
    <div class='quote-container'>
    <blockquote>{}</blockquote>
    <br /> <strong>{}</strong> ({})
    </div>
    <br />
    <a href='http://rocket.rs'>Rocket</a> |
    <a href='http://silverwingedseraph.net'>My Blog</a> |
    <a href='http://twitter.com/leotindall'>My Twitter</a> |
    <a href='/json'> JSON version </a>
    </div>
    </body>
    </html>
    ", quote.0, quote.1, source))
}

#[get("/json")]
pub fn json() -> JSON<String> {
    let quote = quotes::get_random_quote().unwrap();
    let source = quotes::get_source_from_quote_as_json(&quote);
    JSON(format!("{{
        \"quote\": \"{}\",
        \"author\": \"{}\",
        \"source\": {}
    }}", quote.0, quote.1, source))
}
