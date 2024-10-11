use serde_json::{Map, Value};

pub fn get_response_ret(res_json: &Value) -> &str {
    res_json
        .as_object()
        .and_then(|d| d.get("ret"))
        .and_then(|d| d.as_array())
        .and_then(|d| d.get(0))
        .and_then(|d| d.as_str())
        .unwrap()
}

pub fn get_cart_data(cart_json: &Value) -> &Map<String, Value> {
    cart_json
        .as_object()
        .and_then(|d| d.get("data"))
        .and_then(|d| d.as_object())
        .and_then(|o| o.get("data"))
        .and_then(|d| d.as_object())
        .unwrap()
}

pub fn process_cart_items<F>(cart_data: &Map<String, Value>, item_id: &str, mut callback: F)
where
    F: FnMut(&str, &Value),
{
    let cart_data_keys = cart_data.keys();
    let item_keys = cart_data_keys.filter(|&x| x.contains("item_"));
    for k in item_keys {
        if let Some(item) = cart_data.get(k) {
            if item
                .get("fields")
                .and_then(|d| d.get("itemId"))
                .and_then(|d| d.as_str())
                .unwrap()
                .eq(item_id)
            {
                callback(k, item);
            }
        }
    }
}
