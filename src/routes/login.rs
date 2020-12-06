use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub ayp: &'a str
}

#[get("/login")]
pub fn login_page() -> LoginTemplate<'static> {
    LoginTemplate { ayp: "hey there :)" }
}