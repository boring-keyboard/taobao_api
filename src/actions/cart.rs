use crate::actions::utils::{get_cart_data, get_response_ret, process_cart_items};
use crate::cookie::Cookie;
use crate::Api;
use serde_json::Value;

pub fn check_cart(cookie: &Cookie, item_id: &str, api: &Api) -> Value {
    let cart_result = api.get_buy_cart(cookie);

    if let Err(e) = cart_result {
        panic!("{}", e);
    }

    let cart_json = cart_result.unwrap();

    let ret_text = get_response_ret(&cart_json);
    if !ret_text.contains("SUCCESS") {
        panic!("购物车调用失败: {}", ret_text);
    }

    let mut cart_ready_flag = false;
    let cart_data = get_cart_data(&cart_json);
    let mut first: bool = true;
    process_cart_items(cart_data, item_id, |_, item| {
        cart_ready_flag = true;
        print_sku_info(item, first);
        first = false;
    });

    if !cart_ready_flag {
        panic!("购物车未就绪, 先添加商品到购物车");
    }
    println!("----------------------------");
    println!("购物车已就绪");
    cart_json
}

pub fn print_sku_info(sku_info: &Value, show_title: bool) {
    let title = sku_info
        .get("fields")
        .and_then(|d| d.get("title"))
        .and_then(|d| d.as_str())
        .unwrap();

    let sku_title = sku_info
        .get("fields")
        .and_then(|d| d.get("sku"))
        .and_then(|d| d.get("title"))
        .and_then(|d| d.as_str())
        .unwrap();

    let quantity = sku_info
        .get("fields")
        .and_then(|d| d.get("quantity"))
        .and_then(|d| d.as_u64())
        .unwrap();

    if show_title {
        println!("----------------------------");
        println!("{}\n", title);
    }
    println!(
        "{} x{}",
        sku_title, quantity
    );
}
