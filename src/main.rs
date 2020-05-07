use actix_web::{App, HttpServer};

mod admin_meta;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(admin_meta::new_meta)
            .service(admin_meta::update_meta)
            .service(admin_meta::get_meta)
            .service(admin_meta::get_quota)
            .service(admin_meta::new_quota)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
