use askama::Template;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate<'a> {
    pub ayp: &'a str
}

#[get("/register")]
pub fn register_page() -> RegisterTemplate<'static> {
    RegisterTemplate { ayp: "hey there :)" }
}