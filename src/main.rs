use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use derive_more::{AsRef, Deref, DerefMut, Display, From};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, AsRef)]
struct StrWrapper(String);

impl StrWrapper {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[get("/{path:.**}")]
async fn hello(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(path.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let int = StrWrapper(String::new());
    let s = int.to_string();

    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
