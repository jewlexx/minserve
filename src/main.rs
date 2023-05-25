use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            Files::new("/", "./")
                .index_file("index.html")
                .prefer_utf8(true)
                .use_hidden_files()
                .show_files_listing(),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
