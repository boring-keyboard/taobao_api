use crate::actions::utils::{get_cart_data, process_cart_items, get_response_ret};
use serde_json::Value;
use crate::Api;
use crate::cookie::Cookie;

fn parse_settlement_params(cart_json: &Value, item_id: &str) -> String {
    let cart_data = get_cart_data(&cart_json);
    let mut settlement_vec: Vec<String> = Vec::new();
    process_cart_items(&cart_data, item_id, |_, item: &Value| {
        let settlement_str = item
            .get("fields")
            .and_then(|d| d.get("settlement"))
            .and_then(|d| d.as_str())
            .unwrap();
        settlement_vec.push(String::from(settlement_str));
    });
    let settlement_str = settlement_vec.join(",");
    format!("{{\"buyNow\":false,\"buyParam\":\"{}\",\"spm\":\"a21202.12579950.settlement-bar.0\"}}", settlement_str)
}

pub fn settle(cookie: &Cookie, cart_json: &Value, item_id: &str, api: &Api) -> Value {
    let buy_now = parse_settlement_params(cart_json, item_id);

    // println!("{:?}", buy_now);
    let settle_result = api.settle(cookie, &buy_now);

    if let Err(e) = settle_result {
        panic!("{}", e);
    }

    let settle_json = settle_result.unwrap();

    let ret_text = get_response_ret(&settle_json);
    if !ret_text.contains("SUCCESS") {
        panic!("结算调用失败: {}", ret_text)
    }
    settle_json
}