use actix_web::{App, HttpServer};

mod admin_meta;
mod admin_quota;
pub mod entity;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(admin_meta::new_meta)
            .service(admin_meta::update_meta)
            .service(admin_meta::get_meta)
            .service(admin_quota::get_quota)
            .service(admin_quota::new_quota)
            .service(admin_quota::delete_quota)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
