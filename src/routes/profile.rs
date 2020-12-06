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
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub id: String,
    pub username: String,
    pub creationdate: String,
    pub displayname: String,
    pub pronouns: String,
    pub description: String,
    pub birthday: String,
    pub followers: i32,
    pub postsamount: i32,
    pub likedamount: i32,
    pub followingamount: i32,
    pub pfp: String,
    pub banner: String,
    pub current_username: String
}

#[get("/<username>")]
pub fn profile_page(username: String, cookies: Cookies) -> ProfileTemplate {
    let current_user_id = &cookies.get("user_id").unwrap().value();
    let current_jwt = &cookies.get("jwt").unwrap().value();

    let current_user_data_url = Url::parse("http://localhost:8000/api/users/").unwrap().join(&current_user_id).unwrap();
    let current_user_data = reqwest::blocking::get(current_user_data_url).unwrap()
        .json::<ProfileTemplateIncoming>().unwrap();

    let user_id_url = Url::parse("http://localhost:8000/api/users/getid/").unwrap().join(&username).unwrap();

    println!("{}", user_id_url.as_str());

    let user_id = reqwest::blocking::get(user_id_url).unwrap()
        .text().unwrap();

    let body_url = Url::parse("http://localhost:8000/api/users/").unwrap().join(&user_id).unwrap();

    let body = reqwest::blocking::get(body_url).unwrap()
        .json::<ProfileTemplateIncoming>().unwrap();

    ProfileTemplate{
        id: body.id,
        username: body.username,
        creationdate: "".to_string(),
        displayname: body.displayname,
        pronouns: body.pronouns,
        description: body.description,
        birthday: "".to_string(),
        followers: body.followers,
        postsamount: body.posts.len() as i32,
        likedamount: body.likedposts.len() as i32,
        pfp: body.pfp,
        banner: body.banner,
        followingamount: body.following.len() as i32,
        current_username: current_user_data.username
    }
}