use askama::Template;
use reqwest::Url;
use rocket::http::Cookies;

#[derive(Serialize, Deserialize)]
pub struct ProfileTemplateIncoming {
    pub id: String,
    pub username: String,
    pub creationdate: String,
    pub displayname: String,
    pub pronouns: String,
    pub description: String,
    pub birthday: String,
    pub followers: i32,
    pub posts: Vec<String>,
    pub likedposts: Vec<String>,
    pub following: Vec<String>,
    pub pfp: String,
    pub banner: String
}

#[derive(Template)]
#[template(path = "app.html")]
pub struct ProfileTemplate {
    pub current_username: String,
    pub posts: Vec<Post>
}

pub struct Post {
    pub content: String,
}

#[get("/home")]
pub fn home_page(cookies: Cookies) -> ProfileTemplate {
    let current_user_id = &cookies.get("user_id").unwrap().value();
    let current_jwt = &cookies.get("jwt").unwrap().value();

    let current_user_data_url = Url::parse("http://localhost:8000/api/users/").unwrap().join(&current_user_id).unwrap();
    let current_user_data = reqwest::blocking::get(current_user_data_url).unwrap()
        .json::<ProfileTemplateIncoming>().unwrap();
    
    ProfileTemplate {
        current_username: current_user_data.username,
        posts: vec![]
    }
}