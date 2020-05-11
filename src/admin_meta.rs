use crate::response::ResponseBody;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// new meta

#[post("/api/admin/meta")]
pub async fn new_meta() -> impl Responder {
    HttpResponse::Ok().json(ResponseBody::<()>::new_success(None))
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
