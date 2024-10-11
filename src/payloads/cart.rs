use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct ExParam {
    support_calculate_update: String,
    merge_combo: String,
    global_sell: String,
    spi_page: String
}

impl ExParam {
    fn new() -> Self {
        ExParam {
            support_calculate_update: String::from("true"), 
            merge_combo: String::from("true"),
            global_sell: String::from("1"),
            spi_page: String::from("pcTao")
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Payload {
    is_page: bool,
    pub cart_from: String,
    ext_status: String,
    net_type: u8,
    mixed: bool,
    ex_params: String
}

impl Payload {
    pub fn new() -> Self {
        Payload {
            is_page: true,
            ext_status: String::from("0"),
            cart_from: String::from("main_site"), 
            net_type: 0,
            mixed: false,
            ex_params: serde_json::to_string(&ExParam::new()).unwrap()
        }
    }
}


#[derive(Serialize)]
#[serde(rename_all="camelCase")]
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
    callback: String,
    data: String
}

impl Param {
    pub fn new(app_key: &str, t: &str, sign: &str, data: &str) -> Self {
        Param {
            jsv: String::from("2.5.1"),
            app_key: String::from(app_key),
            t: String::from(t),
            sign: String::from(sign),
            api: String::from("mtop.trade.query.bag"),
            v: String::from("5.0"),
            r#type: String::from("jsonp"),
            ttid: String::from("h5"),
            is_sec: String::from("0"),
            ecode: String::from("1"),
            anti_flood: String::from("true"),
            anti_creep: String::from("true"),
            h5_request: String::from("true"),
            data_type: String::from("jsonp"),
            callback: String::from("mtopjsonp2"),
            data: String::from(data)
        }
    }
}
