use askama::Template;

// Homepage
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomePage {}

// Hello
#[derive(Template)]
#[template(
    ext = "html",
    source = "<h1>Hello, {{ name }}!</h1><h2>How are you?</h2>"
)]
pub struct HelloTemplate {
    pub name: String,
}
