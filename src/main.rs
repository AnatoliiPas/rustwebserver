use actix_web::{App, HttpServer};
use utils::views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(views::views_factory))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
