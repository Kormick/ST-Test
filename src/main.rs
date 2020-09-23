mod actix_app;
mod assignment;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_app::run_actix_app().await
}