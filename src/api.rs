use crate::cookie::Cookie;
use crate::payloads;
use crate::payloads::urlencode::UrlParams;
use md5;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, COOKIE, ORIGIN, USER_AGENT};
use rookie::enums::Cookie as RCookie;
use serde_json::Value;
use std::time;

const APP_KEY: &str = "12574478";
const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.36";
const QUERY_BAG_BASE_URL: &str = "https://h5api.m.taobao.com/h5/mtop.trade.query.bag/5.0/";
const ORDER_BUILD_BASE_URL: &str = "https://h5api.m.taobao.com/h5/mtop.trade.order.build.h5/4.0/";
const ORDER_CREATE_BASE_URL: &str = "https://h5api.m.taobao.com/h5/mtop.trade.order.create.h5/4.0/";
const USER_INFO_BASE_URL: &str = "https://h5api.m.taobao.com/h5/mtop.user.getusersimple/1.0/";
const COOKIE_NAME_M_H5_TK: &str = "_m_h5_tk";

#[derive(Clone)]
pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        // 初始化client
        Api {
            client:  Client::builder()
            .redirect(reqwest::redirect::Policy::limited(20))  // 20次
            .build().unwrap(),
        }
    }

    pub fn get_sign_val(d: &str, m_h5_tk: &str) -> (String, String) {
        let t = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let token = m_h5_tk.split('_').collect::<Vec<&str>>()[0];
        let str_sign = format!("{}&{}&{}&{}", token, t, APP_KEY, d);
        let sign = format!("{:x}", md5::compute(str_sign.as_bytes()));
        (sign, t)
    }


    pub fn get_user_info(&self, cookie: &Cookie) -> Result<serde_json::Value, String> {
        let m_h5_tk = get_cookie(cookie, COOKIE_NAME_M_H5_TK);
        let json_payload = "{}";
        let (sign, t) = Self::get_sign_val(json_payload, m_h5_tk.value.as_str());
        let param =
            payloads::user_info::Param::new(APP_KEY, t.as_str(), sign.as_str());

        let url = format!("{}?{}", USER_INFO_BASE_URL, param.to_url_params());
        let headers = build_headers(cookie);

        let response = self.client.post(url).headers(headers).body("data=%7B%7D").send();

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            // println!("Response Body: {}", body);
                            let body_json: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
                            let ret = body_json.get("ret").unwrap().as_array().unwrap().get(0).unwrap().as_str().unwrap();
                            if ret.contains("SUCCESS") {
                                return Ok(body_json);
                            } else {
                                return Err(format!("请求mtop.user.getusersimple失败, ret: {}", ret));
                            }
                        }
                        Err(e) => {
                            return Err(format!("解析用户信息失败: {}", e));
                        }
                    }
                }
                return Err(format!("请求mtop.user.getusersimple失败, 状态码错误: {}", response.status().as_str()));
            }
            Err(e) => {
                return Err(format!("请求mtop.user.getusersimple失败: {}", e));
            }
        }
    }

    pub fn get_buy_cart(&self, cookie: &Cookie) -> Result<serde_json::Value, String> {
        let m_h5_tk = get_cookie(cookie, COOKIE_NAME_M_H5_TK);
        let payload: payloads::cart::Payload = payloads::cart::Payload::new();
        let json_payload = serde_json::to_string(&payload).unwrap();
        let (sign, t) = Self::get_sign_val(json_payload.as_str(), m_h5_tk.value.as_str());
        let param =
            payloads::cart::Param::new(APP_KEY, t.as_str(), sign.as_str(), json_payload.as_str());

        let url = format!("{}?{}", QUERY_BAG_BASE_URL, param.to_url_params());
        let headers = build_headers(cookie);

        let response = self.client.get(url).headers(headers).send().unwrap();

        if response.status().is_success() {
            let body = String::from(response.text().unwrap().trim());
            // println!("Response Body: {}", body);
            let chars: Vec<char> = body.chars().collect();
            // 跳过mtopjsonp2(
            let start = "mtopjsonp2(".len();
            let end = chars.len() - 1;

            let sliced: String = chars[start..end].iter().collect();
            // println!("Response Sliced: {}", sliced);
            return Ok(serde_json::from_str(sliced.as_str()).unwrap());
        }

        return Err(String::from("请求mtop.trade.query.bag失败"));
    }

    pub fn settle(&self, cookie: &Cookie, buy_now: &str) -> Result<serde_json::Value, String> {
        let m_h5_tk = get_cookie(cookie, COOKIE_NAME_M_H5_TK);
        let (sign, t) = Self::get_sign_val(buy_now, m_h5_tk.value.as_str());
        let param: payloads::settlement::Param =
            payloads::settlement::Param::new(APP_KEY, t.as_str(), sign.as_str());
        let url = format!("{}?{}", ORDER_BUILD_BASE_URL, param.to_url_params());
        let headers = build_headers(cookie);
        let body = payloads::settlement::Payload::new(buy_now).to_url_params();

        let response = self
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .unwrap();

        if response.status().is_success() {
            let body = String::from(response.text().unwrap().trim());
            // println!("Response Body: {}", body);
            return Ok(serde_json::from_str(body.as_str()).unwrap());
        }

        return Err(String::from("请求mtop.trade.order.build.h5失败"));
    }

    pub fn submit(
        &self,
        cookie: &Cookie,
        settle_json: &Value,
        params_data: &str,
    ) -> Result<serde_json::Value, String> {
        let payload_data_params =
            payloads::submit::PayloadDataParams::new(settle_json, params_data);
        let payload_data_params_json_str = serde_json::to_string(&payload_data_params).unwrap();
        let payload_data = payloads::submit::PayloadData::new(&payload_data_params_json_str);
        let payload_data_json_str = serde_json::to_string(&payload_data).unwrap();
        let m_h5_tk = get_cookie(cookie, COOKIE_NAME_M_H5_TK);
        let (sign, t) = Self::get_sign_val(&payload_data_json_str, m_h5_tk.value.as_str());
        let url_param = payloads::submit::Param::new(APP_KEY, t.as_str(), sign.as_str());
        let url = format!("{}?{}", ORDER_CREATE_BASE_URL, url_param.to_url_params());
        let body = payloads::submit::Payload::new(&payload_data_json_str).to_url_params();

        let headers = build_headers(cookie);

        let response = self
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .unwrap();

        if response.status().is_success() {
            let body = String::from(response.text().unwrap().trim());
            // println!("Response Body: {}", body);
            return Ok(serde_json::from_str(body.as_str()).unwrap());
        }

        return Err(String::from("请求mtop.trade.order.create.h5失败"));
    }

    fn get_order_create_time(&self, cookie: &Cookie, order_id: &str, url: &str) -> Result<String, String> {
        let headers = build_headers(cookie);

        let response = self.client.get(url).headers(headers).send().unwrap();

        if response.status().is_success() {
            let body = String::from(response.text().unwrap());
            let re = Regex::new(r#""(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})"#).unwrap();
            for line in body.lines().rev() {
                if let Some(caps) = re.captures(line) {
                    println!("订单编号: {}", order_id);
                    println!("创建时间: {}", caps[1].to_string());
                    println!("\n请在客户端完成支付");
                    return Ok(caps[1].to_string());
                }
            }
            return Err(String::from("解析订单创建时间失败"));
        }

        return Err(String::from("请求订单详情失败"));
    }

    fn get_order_create_time_taobao(&self, cookie: &Cookie, order_id: &str) -> Result<String, String> {
        let url = format!(
            "https://trade.taobao.com/trade/detail/trade_order_detail.htm?biz_order_id={}",
            order_id
        );
        self.get_order_create_time(cookie, order_id, &url)
    }

    fn get_order_create_time_tmall(&self, cookie: &Cookie, order_id: &str) -> Result<String, String> {
        let url = format!(
            "https://trade.tmall.com/detail/orderDetail.htm?biz_order_id={}&forward_action=",
            order_id
        );
        self.get_order_create_time(cookie, order_id, &url)
    }

    pub fn get_order_create_time_mix(&self, cookie: &Cookie, order_id: &str) -> Result<String, String> {
        let result = self.get_order_create_time_taobao(cookie, order_id);
        if result.is_err() {
            return self.get_order_create_time_tmall(cookie, order_id);
        }
        return result;
    }
}

fn get_cookie<'a>(cookie: &'a Cookie, name: &str) -> &'a RCookie {
    cookie
        .cookies
        .iter()
        .find(|&x| x.name == name)
        .expect(format!("获取cookie失败name:{}", name).as_str())
}

fn build_headers(cookie: &Cookie) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        ORIGIN,
        HeaderValue::from_static("https://main.m.taobao.com"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    let cookie_str = cookie.to_cookies_str(".taobao.com");
    headers.insert(COOKIE, HeaderValue::from_str(cookie_str.as_str()).unwrap());
    headers
}
