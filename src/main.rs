use actix_web::{App, HttpServer};
use log::Level;
use std::env;

mod admin_meta;
mod admin_quota;
mod config_path;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();

    HttpServer::new(|| {
        App::new()
            .data(config_path::get_db())
            .data(config_path::ConfigPath::default())
            .service(admin_meta::new_cert)
            .service(admin_meta::register_cms)
            .service(admin_meta::update_cert)
            .service(admin_meta::get_cert)
            .service(admin_quota::new_quota)
            .service(admin_quota::delete_quota)
            .service(admin_quota::convert_quota)
    })
    .bind(&args[1])?
    .run()
    .await
}
