use crate::actions::utils::get_response_ret;
use crate::cookie::Cookie;
use crate::Api;
use serde_json::{json, Value};

pub fn parse_params_data(settle_json: &Value) -> String {
    let item_like_arr = [
        "item_",
        "itemInfo_",
        "service_yfx_",
        "invoice_",
        "promotion_",
        "deliveryDate_",
    ];
    let item_hit_arr = [
        "anonymous_1",
        "address_1",
        "voucher_1",
        "confirmOrder_1",
        "ncCheckCode_ncCheckCode1",
        "submitOrder_1",
    ];

    let mut output_json = json!({});

    if let Value::Object(input_map) = settle_json.get("data").unwrap().get("data").unwrap() {
        for (key, value) in input_map {
            if item_hit_arr.contains(&key.as_str()) {
                output_json[key] = value.clone();
            } else {
                let mut is_prefix = false;
                for item_like in &item_like_arr {
                    if key.starts_with(item_like) {
                        is_prefix = true;
                        break;
                    }
                }
                if is_prefix {
                    output_json[key] = value.clone();
                }
            }
        }
    }
    serde_json::to_string(&output_json).unwrap()
}

pub fn submit(cookie: &Cookie, settle_json: &Value, api: &Api, settle_time: (chrono::DateTime<chrono::Local>, chrono::DateTime<chrono::Local>)) -> (String, Value) {
    let params_data = parse_params_data(settle_json);
    let start_time: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let submit_result = api.submit(cookie, settle_json, params_data);
    println!("{},{}", settle_time.0.format("%H:%M:%S%.3f").to_string(), settle_time.1.format("%H:%M:%S%.3f").to_string());
    println!("{},{}", start_time.format("%H:%M:%S%.3f").to_string(), chrono::Local::now().format("%H:%M:%S%.3f").to_string());
    if let Err(e) = submit_result {
        panic!("{}", e);
    }

    let submit_json = submit_result.unwrap();
    let ret_text = get_response_ret(&submit_json);
    if !ret_text.contains("SUCCESS") {
        panic!("提单调用失败: {}", ret_text)
    }

    let order_id = submit_json
        .get("data")
        .and_then(|d| d.get("bizOrderId"))
        .and_then(|d| d.as_str())
        .unwrap()
        .to_string();
    (order_id, submit_json)
}
