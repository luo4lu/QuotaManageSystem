use crate::config_path::ConfigPath;
use crate::response::ResponseBody;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sha3::Sha3;
use asymmetric_crypto::keypair::Keypair;
use core::convert::AsRef;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use hex::{FromHex, ToHex};
use std::fmt::Write;

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
pub async fn update_meta(config: web::Data<ConfigPath>, req: web::Json<UpdateMetaRequest>) -> impl Responder {
    let sd: [u8; 32] = FromHex::from_hex(&req.seed).unwrap();
    let info_form_rang = Keypair::<
    [u8; 32],
    Sha3,
    dislog_hal_sm2::PointInner,
    dislog_hal_sm2::ScalarInner,
>::generate_from_seed(sd).unwrap();
let serialized = serde_json::to_string(&info_form_rang).unwrap();
let mut file = File::create(&config.meta_path).await.unwrap();
match file.write_all(serialized.as_ref()).await{
    Ok(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_success(None)),
    Err(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_json_parse_error()),
}
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
pub async fn get_meta(config: web::Data<ConfigPath>) -> impl Responder {
    //read file
    let mut file = File::open(&config.meta_path).await.unwrap();
    //read json file to string
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    //Deserialize to the specified data format
    let deserialize: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
        > = serde_json::from_str(&contents).unwrap();

        //format conversion to string
        let mut secret_str = String::new();
        let mut code_str = String::new();
        let mut seed_str = String::new();
        for a in deserialize.get_secret_key().to_bytes().iter(){
            write!(secret_str,"{:02x}",a).unwrap();
        }
        let  public_str = deserialize.get_public_key().to_bytes().encode_hex();
        for a in deserialize.get_code().iter(){
            write!(code_str,"{:02x}",a).unwrap();
        }
        for a in deserialize.get_seed().iter(){
            write!(seed_str,"{:02x}",a).unwrap();
        }
    HttpResponse::Ok().json(ResponseBody::new_success(Some(GetMetaResponse {
        code: code_str,
        public: public_str,
        secret: secret_str,
        seed: seed_str,
    })))
}
