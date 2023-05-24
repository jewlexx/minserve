use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/{path}")]
async fn hello(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
