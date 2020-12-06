use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate<'a> {
    pub ayp: &'a str
}

#[get("/about")]
pub fn about_page() -> AboutTemplate<'static> {
    AboutTemplate { ayp: "hey there :)" }
}