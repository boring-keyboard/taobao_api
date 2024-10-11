use rookie::chrome;
use rookie::edge;
use rookie::enums::Cookie as RCookie;
use rookie::firefox;
// use rookie::safari;

pub struct Cookie {
    pub cookies: Vec<RCookie>,
}

impl Cookie {
    pub fn new(browser: &str) -> Self {
        let mut cookie = Cookie {
            cookies: Vec::new()
        };
        cookie.extract_cookies(browser);
        return cookie;
    }
    fn extract_cookies(&mut self, browser: &str) -> () {
        let domains = vec!["taobao.com"];
        let domains: Vec<String> = domains.iter().map(|&s| s.to_string()).collect();
        let cookies;
        match browser {
            "chrome" => {
                match chrome(Some(domains)) {
                    Ok(chrome_cookies) => cookies = chrome_cookies,
                    Err(e) => {
                        panic!("Chrome浏览器解析失败: {}", e);
                    }
                }
            }
            // "safari" => {
            //     cookies = safari(Some(domains)).unwrap();
            // }
            "firefox" => {
                match firefox(Some(domains)) {
                    Ok(firefox_cookies) => cookies = firefox_cookies,
                    Err(e) => {
                        panic!("Firefox浏览器解析失败: {}", e);
                    }
                }
            }
            "edge" => {
                match edge(Some(domains)) {
                    Ok(edge_cookies) => cookies = edge_cookies,
                    Err(e) => {
                        panic!("Edge浏览器解析失败: {}", e);
                    }
                }
            }
            _ => {
                println!("未知浏览器");
                cookies = Vec::new(); // Initialize cookies with an empty vector
            }
        }
        self.cookies = cookies;
    }

    pub fn to_cookies_str(&self, domain: &str) -> String {
        let mut cookie_item_vec: Vec<String>  = Vec::new();
        for cookie in &self.cookies {
            if cookie.domain.eq(domain) {
                cookie_item_vec.push(format!("{}={}", cookie.name, cookie.value));
            }
        }

        cookie_item_vec.join("; ")
    }
}
