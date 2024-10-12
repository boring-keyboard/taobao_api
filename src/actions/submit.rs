use crate::actions::utils::get_response_ret;
use crate::cookie::Cookie;
use crate::Api;
use core::panic;
use serde_json::{json, Value};
use std::sync::{mpsc, Arc};
use std::thread;

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

/**
 * 提单策略
 * 前3次并发提交，每次间隔33ms发送请求
 * 如果3次都失败，再请求10次，每次发送都等待上次请求返回结果
 */
pub fn submit(
    cookie: &Cookie,
    settle_json: &Value,
    api: &Api,
    settle_time: (
        chrono::DateTime<chrono::Local>,
        chrono::DateTime<chrono::Local>,
    ),
    is_test: bool,
) -> (String, Value) {
    let params_data = parse_params_data(settle_json);

    let cookie = Arc::new(cookie.clone());
    let settle_json = Arc::new(settle_json.clone());
    let api = Arc::new(api.clone());
    let params_data = Arc::new(params_data);

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);
    let mut handles = Vec::new();

    let times = if is_test { 1 } else { 3 };

    for _ in 0..times {
        let tx = Arc::clone(&tx);
        let cookie = Arc::clone(&cookie);
        let settle_json = Arc::clone(&settle_json);
        let api = Arc::clone(&api);
        let params_data = Arc::clone(&params_data);

        handles.push(thread::spawn(move || {
            let one_submit_result = api.submit(&cookie, &settle_json, &params_data);

            match tx.send(one_submit_result) {
                Ok(_) => (),
                Err(e) => println!("线程发送结果失败 {}", e),
            }
        }));
        thread::sleep(std::time::Duration::from_millis(33));
    }

    println!(
        "结算 {},{}",
        settle_time.0.format("%H:%M:%S%.3f").to_string(),
        settle_time.1.format("%H:%M:%S%.3f").to_string()
    );

    for submit_result in rx {
        println!("{:?}", submit_result);
        match check_submit_result(submit_result) {
            Ok((order_id, submit_json)) => {
                return (order_id, submit_json);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 再请求10次
    for _ in 0..10 {
        let one_submit_result: Result<Value, String> =
            api.submit(&cookie, &settle_json, &params_data);
        match check_submit_result(one_submit_result) {
            Ok((order_id, submit_json)) => {
                return (order_id, submit_json);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    panic!("提单失败");
}

fn check_submit_result(submit_result: Result<Value, String>) -> Result<(String, Value), String> {
    match submit_result {
        Ok(submit_json) => {
            let ret_text = get_response_ret(&submit_json);
            if !ret_text.contains("SUCCESS") {
                return Err(format!("提单调用失败: {}", ret_text));
            } else {
                let order_id = submit_json
                    .get("data")
                    .and_then(|d| d.get("bizOrderId"))
                    .and_then(|d| d.as_str())
                    .unwrap()
                    .to_string();
                return Ok((order_id, submit_json));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}
