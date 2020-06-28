use actix_web::{App, HttpServer};
use clap::ArgMatches;

mod admin_meta;
mod admin_quota;
mod config_command;
mod config_path;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut _path: String = String::new();
    let matches: ArgMatches = config_command::get_command();
    if let Some(d) = matches.value_of("qms") {
        _path = d.to_string();
    } else {
        _path = String::from("127.0.0.1:9003");
    }
    //Initialize the log and set the print level
    // simple_logger::init_with_level(Level::Warn).unwrap();
    env_logger::init();
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
    .bind(_path)?
    .run()
    .await
}
