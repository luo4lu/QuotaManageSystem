use crate::config_path::ConfigPath;
use crate::response::ResponseBody;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sha3::Sha3;
use asymmetric_crypto::keypair::Keypair;
use core::convert::AsRef;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
//use hex::FromHex;
//use std::fmt::Write;

use tokio::fs::File;
use tokio::prelude::*;

// new meta

#[post("/api/admin/meta")]
pub async fn new_meta(config: web::Data<ConfigPath>) -> impl Responder {
    //decline a rand number object
    let mut rng = thread_rng();
    //generate Serialize structure data
    let info_form_rang = Keypair::<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    >::generate(&mut rng)
    .unwrap();
    let serialized = serde_json::to_string(&info_form_rang).unwrap();
    let mut file = File::create(&config.meta_path).await.unwrap();
    match file.write_all(serialized.as_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_success(None)),
        Err(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_json_parse_error()),
    }
}

// update meta
#[derive(Deserialize, Debug)]
pub struct UpdateMetaRequest {
    seed: String,
}

#[put("/api/admin/meta")]
pub async fn update_meta(req: web::Json<UpdateMetaRequest>) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().json(ResponseBody::<()>::new_success(None))
}

// get meta

#[derive(Serialize)]
pub struct GetMetaResponse {
    code: String,
    public: String,
    secret: String,
    seed: String,
}

#[get("/api/admin/meta")]
pub async fn get_meta() -> impl Responder {
    HttpResponse::Ok().json(ResponseBody::new_success(Some(GetMetaResponse {
        code: String::from(""),
        public: String::from(""),
        secret: String::from(""),
        seed: String::from(""),
    })))
}
