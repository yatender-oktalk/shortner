use actix_web::{get,web,post, HttpResponse,HttpRequest, App, HttpServer, Result, error, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}
/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!(
        "Welcome {}, user_id {}!",
        name, userid
    ))
}

/// extract body info using serde deserialize
#[get("/")] // <- define path parameters
async fn index2(info: web::Query<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {},!",
        info.username,
    ))
}

async fn index4(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

/// extract path info using serde
#[post("/submit")] // <- define path parameters
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {},!",
        info.username,
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

    App::new()
    .service(index)
    .service(index2)
    .service(submit)
    .service(
        web::resource("/userlimit")
        .app_data(json_config)
        .route(web::post().to(index4))
    )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

