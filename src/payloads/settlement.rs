use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    jsv: String,
    app_key: String,
    t: String,
    sign: String,
    api: String,
    v: String,
    r#type: String,
    ttid: String,
    is_sec: String,
    ecode: String,
    #[serde(rename = "AntiFlood")]
    anti_flood: String,
    #[serde(rename = "AntiCreep")]
    anti_creep: String,
    #[serde(rename = "H5Request")]
    h5_request: String,
    data_type: String,
}

impl Param {
    pub fn new(app_key: &str, t: &str, sign: &str) -> Self {
        Param {
            jsv: String::from("2.5.1"),
            app_key: String::from(app_key),
            t: String::from(t),
            sign: String::from(sign),
            api: String::from("mtop.trade.order.build.h5"),
            v: String::from("4.0"),
            r#type: String::from("originaljson"),
            ttid: String::from("#t#ip##_h5_2019"),
            is_sec: String::from("1"),
            ecode: String::from("1"),
            anti_flood: String::from("true"),
            anti_creep: String::from("true"),
            h5_request: String::from("true"),
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