use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result
};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Deserialize, Debug)]
struct UserSignUpRequest {
    email: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSignUpResponse{
    id: String,
    email: String,
    name: String,
}

/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

/// extract path info using serde
#[post("/submit")] // <- define path parameters
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {},!", info.username,))
}

#[post("")]
async fn create_user(req: web::Json<UserSignUpRequest>) -> HttpResponse {
    let info = req.into_inner();
    let create_user_resp = web::Json(UserSignUpResponse{
        id: String::from("2"),
        email: info.name,
        name: info.email,
    });

    HttpResponse::Ok()
    .content_type("application/json")
    .insert_header(("auth-token", "TYUIO98HJIKDKJDLFJOSOJUI"))
    .json(create_user_resp)
}

#[get("/{user_id}")]
async fn show_user(req: HttpRequest) -> impl Responder {
    let user_id: i32 = req.match_info().query("user_id").parse().unwrap();

    format!("Welcome user_id {}!", user_id)
}

#[get("")]
async fn show_all_users() -> impl Responder {
    format!("showing all users")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .service(create_user)
                .service(show_user)
                .service(show_all_users),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
