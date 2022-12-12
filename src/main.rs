use actix_web::{get, HttpRequest, App, HttpServer, Result};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
