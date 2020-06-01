use crate::config_path::ConfigPath;
use crate::response::ResponseBody;
use actix_web::{delete, post, put, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sha3::Sha3;
use asymmetric_crypto::keypair;
use asymmetric_crypto::prelude::Keypair;
use common_structure::convert_quota_request::ConvertQoutaRequestWrapper;
use common_structure::issue_quota_request::IssueQuotaRequestWrapper;
use common_structure::quota_control_field::QuotaControlFieldWrapper;
use dislog_hal::Bytes;
use kv_object::kv_object::MsgType;
use kv_object::prelude::KValueObject;
use kv_object::sm2::KeyPairSm2;
use log::{info, warn};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use hex::{FromHex, ToHex};
use tokio::fs::File;
use tokio::prelude::*;

//数据库相关
use deadpool_postgres::Pool;

//The struct in the array as request
#[derive(Deserialize, Serialize, Debug)]
pub struct NewQuota {
    issue_quota_request: String,
}

#[post("/api/quota")]
pub async fn new_quota(
    data: web::Data<Pool>,
    config: web::Data<ConfigPath>,
    qstr: web::Json<NewQuota>,
) -> impl Responder {
    let mut rng = thread_rng();
    //read file for get seed
    let mut file = match File::open(&config.meta_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file open failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
    //read json file to string
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("read file to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //deserialize to the specified data format
    let keypair_value: keypair::Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = match serde_json::from_str(&contents) {
        Ok(de) => {
            info!("{:?}", de);
            de
        }
        Err(e) => {
            warn!("Keypair generate failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //pass encode hex conversion get seed
    let seed: [u8; 32] = keypair_value.get_seed();
    //get  digital signature
    let keypair_sm2: KeyPairSm2 = KeyPairSm2::generate_from_seed(seed).unwrap();

    //反序列化得到指定格式的值
    let deser_vec = Vec::<u8>::from_hex(&qstr.issue_quota_request).unwrap();
    let issue = IssueQuotaRequestWrapper::from_bytes(&deser_vec).unwrap();

    //验证签名
    if issue.verfiy_kvhead().is_ok() {
        info!("true");
    } else {
        warn!("quota issue request verfiy check failed");
        return HttpResponse::Ok().json(ResponseBody::<()>::new_json_parse_error());
    }
    //签名后的额度生成请求获取生成信息
    let quotas = issue.get_body().quota_distribution();
    let mut vec: Vec<String> = Vec::new();
    //存储到数据库
    let conn = data.get().await.unwrap(); //连接到数据库获取连接句柄

    for (index, quota) in quotas.iter().enumerate() {
        let mut quota_control_field =
            QuotaControlFieldWrapper::new(MsgType::QuotaControlField, quota.clone());
        //生成签名
        quota_control_field
            .fill_kvhead(&keypair_sm2, &mut rng)
            .unwrap();
        let sign_bytes = quota_control_field.to_bytes();
        //序列化为十六进制串
        vec.push(sign_bytes.encode_hex::<String>());

        //获取数据库id字段
        let id = (*quota_control_field.get_body().get_id()).encode_hex::<String>();
        //状态
        let state: String = String::from("issued");
        let jsonb_quota = serde_json::to_value(&quota_control_field).unwrap();
        //数据库操作
        let statement = conn
            .prepare(
                "INSERT INTO quota_control_field (id, quota_control_field, explain_info, 
                state, create_time, update_time) VALUES ($1, $2, $3, $4, now(), now())",
            )
            .await
            .unwrap();

        conn.execute(&statement, &[&id, &vec[index], &jsonb_quota, &state])
            .await
            .unwrap();
    }

    HttpResponse::Ok().json(ResponseBody::new_success(Some(vec)))
}

#[delete("/api/quota")]
pub async fn delete_quota(data: web::Data<Pool>, vec: web::Json<Vec<String>>) -> impl Responder {
    format!("{:?}", vec);
    //存储额度控制为ID
    let mut field_id: Vec<String> = Vec::new();
    //连接数据库
    let conn = data.get().await.unwrap();
    //反序列化得到指定格式的值
    for (index, value) in vec.iter().enumerate() {
        let deser_vec = Vec::<u8>::from_hex(value).unwrap();
        let quota_control_field = QuotaControlFieldWrapper::from_bytes(&deser_vec).unwrap();
        //获取额度控制ID并以十六进制输出
        field_id.push((*quota_control_field.get_body().get_id()).encode_hex());
        //数据库命令
        let state: String = String::from("recycle");
        let statement = conn
            .prepare("UPDATE quota_control_field SET state = $1 WHERE id = $2")
            .await
            .unwrap();

        conn.execute(&statement, &[&state, &field_id[index]])
            .await
            .unwrap();
    }
    HttpResponse::Ok().json(ResponseBody::new_success(Some(field_id)))
}

//The struct in the array as request
#[derive(Deserialize, Serialize, Debug)]
pub struct ConvertQuota {
    convert_quota_request: String,
}
#[put("/api/quota")]
pub async fn convert_quota(
    data: web::Data<Pool>,
    config: web::Data<ConfigPath>,
    qstr: web::Json<ConvertQuota>,
) -> impl Responder {
    let mut rng = thread_rng();
    //read file for get seed
    let mut file = match File::open(&config.meta_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file open failed: {:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
    //read json file to string
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("read file to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //deserialize to the specified data format
    let keypair_value: keypair::Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = match serde_json::from_str(&contents) {
        Ok(de) => {
            info!("{:?}", de);
            de
        }
        Err(e) => {
            warn!("Keypair generate failed: {:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //pass encode hex conversion get seed
    let seed: [u8; 32] = keypair_value.get_seed();
    //use seed generate digital signature
    let keypair_sm2: KeyPairSm2 = KeyPairSm2::generate_from_seed(seed).unwrap();

    //输入转换额度请求进行反序列化
    let deser_vec = Vec::<u8>::from_hex(&qstr.convert_quota_request).unwrap();
    let issue = ConvertQoutaRequestWrapper::from_bytes(&deser_vec).unwrap();

    //验证签名
    if issue.verfiy_kvhead().is_ok() {
        info!("true");
    } else {
        warn!("quota issue request verfiy check failed");
        return HttpResponse::Ok().json(ResponseBody::<()>::new_json_parse_error());
    }
    //存储到数据库
    let conn = data.get().await.unwrap(); //连接到数据库获取连接句柄

    //获取已经存在额度控制为信息
    let inputs = issue.get_body().get_inputs();
    //标记销毁之前的额度
    for quota_control in inputs.iter() {
        let old_state: String = String::from("recycle");
        let id = (*quota_control.get_body().get_id()).encode_hex::<String>();
        let select_data = conn
            .prepare("SELECT id from quota_control_field where id = $1")
            .await
            .unwrap();
        match conn.query_one(&select_data, &[&id]).await {
            Ok(row) => {
                info!("{:?}", row);
                row
            }
            Err(error) => {
                warn!("select id failed:{:?}", error);
                return HttpResponse::Ok().json(ResponseBody::<()>::database_build_error());
            }
        };
        let delete_data = conn
            .prepare("UPDATE quota_control_field SET state = $1,update_time = now() WHERE id = $2")
            .await
            .unwrap();
        conn.execute(&delete_data, &[&old_state, &id])
            .await
            .unwrap();
    }
    //签名后的转换额度生成请求获取生成信息
    let quotas = match issue.get_body().convert() {
        Ok(q) => q,
        Err(_err) => {
            warn!("quota convert failed");
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //初始化一个容器保存响应数据
    let mut vec: Vec<String> = Vec::new();
    for (index, quota) in quotas.iter().enumerate() {
        let mut quota_control_field =
            QuotaControlFieldWrapper::new(MsgType::QuotaControlField, quota.clone());
        //生成签名
        quota_control_field
            .fill_kvhead(&keypair_sm2, &mut rng)
            .unwrap();
        let sign_bytes = quota_control_field.to_bytes();
        //序列化为十六进制
        vec.push(sign_bytes.encode_hex::<String>());

        //获取数据库id字段
        let id = (*quota_control_field.get_body().get_id()).encode_hex::<String>();
        //状态值
        let new_state: String = String::from("issued");
        let jsonb_quota = serde_json::to_value(&quota_control_field).unwrap();
        //插入新的数据
        let insert_data = conn
            .prepare(
                "INSERT INTO quota_control_field (id, quota_control_field, explain_info,
            state, create_time, update_time) VALUES ($1, $2, $3, $4, now(), now())",
            )
            .await
            .unwrap();
        conn.execute(&insert_data, &[&id, &vec[index], &jsonb_quota, &new_state])
            .await
            .unwrap();
    }
    HttpResponse::Ok().json(ResponseBody::new_success(Some(vec)))
}
