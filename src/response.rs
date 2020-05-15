use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBody<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ResponseBody<T> {
    pub fn new_success(data: Option<T>) -> Self {
        ResponseBody {
            code: 0,
            message: String::from("success"),
            data,
        }
    }

    pub fn new_json_parse_error() -> Self {
        ResponseBody {
            code: 90001,
            message: String::from("json parse error."),
            data: None,
        }
    }

    //文件类错误
    pub fn new_file_error() -> Self {
        ResponseBody {
            code: 500,
            message: String::from("file open or write or read error."),
            data: None,
        }
    }

    //字符转换
    pub fn new_str_conver_error() -> Self {
        ResponseBody {
            code: 7000,
            message: String::from("char conversion error"),
            data: None,
        }
    }
}
