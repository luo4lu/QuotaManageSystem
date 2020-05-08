use crate::response::ResponseBody;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
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

#[get("/api/quota")]
pub async fn get_quota() -> impl Responder {
    HttpResponse::Ok().json(ResponseBody::<String>::new_success(Some(String::from(
        "dadaswsda",
    ))))
}

//The struct in the array as request
#[derive(Deserialize, Debug)]
pub struct NewQuota {
    count: i32,
    amount: i64,
}

/*impl NewQuota{
    pub fn new_vec() ->Vec<NewQuota>{
        let NewQuotaReq: Vec<NewQuota> = Vec::new();
        NewQuotaReq = vec![NewQuota{count: 1, amount: 800000000},NewQuota{count: 2, amount: 900000000}];
        NewQuotaReq
    }
}*/

#[post("/api/quota")]
pub async fn new_quota(vec: web::Json<Vec<NewQuota>>) -> impl Responder {
    println!("{:?}", vec);
    let v = vec!["0x00001", "0x00002", "0x00003"];
    HttpResponse::Ok().json(ResponseBody::new_success(Some(v)))
}

#[delete("/api/quota")]
pub async fn delete_quota(vec: web::Json<Vec<String>>) -> impl Responder {
    println!("{:?}", vec);
    let v = vec!["0x00001", "0x00002", "0x00003"];
    HttpResponse::Ok().json(ResponseBody::new_success(Some(v)))
}