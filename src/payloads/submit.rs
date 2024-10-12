use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    jsv: String,
    app_key: String,
    t: String,
    sign: String,
    api: String,
    v: String,
    post: String,
    r#type: String,
    timeout: String,
    ttid: String,
    is_sec: String,
    ecode: String,
    #[serde(rename = "AntiFlood")]
    anti_flood: String,
    data_type: String,
}

impl Param {
    pub fn new(app_key: &str, t: &str, sign: &str) -> Self {
        Param {
            jsv: String::from("2.5.1"),
            app_key: String::from(app_key),
            t: String::from(t),
            sign: String::from(sign),
            api: String::from("mtop.trade.order.create.h5"),
            v: String::from("4.0"),
            post: String::from("1"),
            r#type: String::from("originaljson"),
            timeout: String::from("15000"),
            ttid: String::from("#t#ip##_h5_2019"),
            is_sec: String::from("1"),
            ecode: String::from("1"),
            anti_flood: String::from("true"),
            data_type: String::from("jsonp"),
        }
    }
}


#[derive(Serialize)]
pub struct Payload {
    data: String,
}

impl Payload {
    pub fn new(data: &str) -> Self {
        Payload {
            data: String::from(data)
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadData {
    params: String,
}

impl PayloadData {
    pub fn new(data: &str) -> Self {
        PayloadData {
            params: String::from(data),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadDataParams {
    operator: Option<String>,
    data: String,
    linkage: String,
    hierarchy: String,
    lifecycle: Option<String>,
}

impl PayloadDataParams {
    pub fn new(settle_json: &Value, data: &str) -> Self {
        PayloadDataParams {
            operator: None,
            data: String::from(data),
            linkage: serde_json::to_string(settle_json.get("data").unwrap().get("linkage").unwrap()).unwrap(),
            hierarchy: serde_json::to_string(settle_json.get("data").unwrap().get("hierarchy").unwrap()).unwrap(),
            lifecycle: None
        }    
    }
}