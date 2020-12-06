use rocket::http::{Cookies, Cookie};
use rocket::response::Redirect;

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    reqwest::blocking::get("http://localhost:8000/api/users/logout").unwrap();

    cookies.remove(Cookie::named("user_id"));
    cookies.remove(Cookie::named("jwt"));

    Redirect::to("/")
}