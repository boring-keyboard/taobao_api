mod api;
mod cookie;
mod guide;
mod terminal;
mod payloads {
    pub mod cart;
    pub mod settlement;
    pub mod submit;
    pub mod urlencode;
    pub mod user_info;
}
mod actions {
    pub mod cart;
    pub mod settlement;
    pub mod submit;
    mod utils;
}
mod sync_time;
use api::Api;
use cookie::Cookie;
use guide::Guide;
use guide::Position;
use std::panic;
use terminal::Terminal;

fn main() -> () {
    panic::set_hook(Box::new(|_info| {
        let payload = _info.payload();
        if let Some(s) = payload.downcast_ref::<&str>() {
            println!("程序异常退出 {}", s);
        } else if let Some(s) = payload.downcast_ref::<String>() {
            println!("程序异常退出 {}", s);
        } else {
            println!("程序异常退出");
        }
        #[cfg(debug_assertions)]
        println!(">>>>>>>>>>>>>>>>>>>\n{}\n>>>>>>>>>>>>>>>>>>>", _info);

        Guide::wait_for_any_key();
    }));

    let browser = Guide::default().run();
    if browser.is_empty() {
        return;
    }

    let cookie = get_cookie(browser.as_str());
    let api = Api::new();
    
    check_user_permission(&api, &cookie);

    let is_test = Guide::select_is_test();

    let item_id = Guide::input_item_id();
    let cart_json = actions::cart::check_cart(&cookie, &item_id, &api);

    let mut settle_time = (chrono::Local::now(), chrono::Local::now());

    let network_delay_ms = sync_time::calc_network_delay(&api, &cookie);
    let use_delay = Guide::input_use_delay();
    let start_time = Guide::input_start_time();
    sync_time::wait_until(&start_time, network_delay_ms, use_delay);
    settle_time.0 = chrono::Local::now();
    let settle_json = actions::settlement::settle(&cookie, &cart_json, &item_id, &api);
    settle_time.1 = chrono::Local::now();
    let (order_id, _) = actions::submit::submit(&cookie, &settle_json, &api, settle_time, is_test);

    println!("已提单, 查询订单信息...");
    if let Err(e) = api.get_order_create_time_mix(&cookie, &order_id) {
        panic!("{}", e);
    }
    Guide::wait_for_any_key();
}

fn get_cookie(browser: &str) -> Cookie {
    Cookie::new(browser)
}

fn check_user_permission(api: &Api, cookie: &Cookie) -> () {
    let user = api.get_user_info(cookie).expect("获取用户信息失败");
    // println!("用户: {}", user);
    let nick = user
        .get("data")
        .and_then(|d| d.get("nick"))
        .and_then(|d| d.as_str())
        .unwrap();
    let display_nick = user
        .get("data")
        .and_then(|d| d.get("displayNick"))
        .and_then(|d| d.as_str())
        .unwrap();

    let auth_user = vec![
        "nuannuan44777",
        "lhy5246",
        "枝寄禾_雾",
        "繁花映星宇",
        "tb485515046",
        "tb861675844",
        "神淼o0",
        "tb325084889061",
        "魅影邪君丨"
    ];

    if !(auth_user.contains(&nick) || auth_user.contains(&display_nick)) {
        panic!("未授权taobao账号<{}>", nick);
    }
    println!("已授权taobao账号<{}>", nick);
}