use serde::Serialize;


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
    prevent_fallback: String,
    data_type: String,
}

impl Param {
    pub fn new(app_key: &str, t: &str, sign: &str) -> Self {
        Param {
            jsv: String::from("2.7.2"),
            app_key: String::from(app_key),
            t: String::from(t),
            sign: String::from(sign),
            api: String::from("mtop.user.getUserSimple"),
            v: String::from("5.0"),
            r#type: String::from("originaljson"),
            prevent_fallback: String::from("true"),
            data_type: String::from("jsonp"),
        }
    }
}
