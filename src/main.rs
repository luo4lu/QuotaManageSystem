use actix_web::{App, HttpServer};
use log::Level;
use simple_logger;

mod admin_meta;
mod admin_quota;
mod config_path;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();

    HttpServer::new(|| {
        App::new()
            .data(config_path::ConfigPath::default())
            .service(admin_meta::new_cert)
            .service(admin_meta::update_cert)
            .service(admin_meta::get_cert)
            .service(admin_quota::get_quota)
            .service(admin_quota::new_quota)
            .service(admin_quota::delete_quota)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
