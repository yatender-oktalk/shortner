use actix_web::{get, web, App, HttpServer, Result};

#[get("/users/{user_id}/{friend}")]
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}", friend, user_id))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
