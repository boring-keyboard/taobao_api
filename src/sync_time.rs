use rsntp::SntpClient;
use chrono::TimeZone;
use chrono::{NaiveDateTime, Local};
use std::time::Duration;
use crate::api::Api;
use crate::cookie::Cookie;
use crate::terminal::Terminal;
use std::thread::sleep;

pub fn sync() -> f64 {
    let client = SntpClient::new();
    let result = client.synchronize("ntp.aliyun.com").unwrap();
    
    println!("同步阿里云服务器时间: {:.4}secs", result.clock_offset().as_secs_f64());
    return result.clock_offset().as_secs_f64();
}

pub fn calc_network_delay(api: &Api, cookie: &Cookie) -> f64 {
    // 假设网络延迟服从正态分布
    // 请求10次, 延迟=（接收时间-发送时间）/2
    println!("正在计算网络延迟...");
    let times = 10;
    let mut delay_vec = Vec::new();
    for _ in 0..times {
        let start = Local::now();
        let _ = api.get_user_info(cookie);
        let end = Local::now();
        let delay = (end.timestamp_millis() - start.timestamp_millis()) as f64 / 2.0;
        delay_vec.push(delay);
        println!("网络延迟: {:.4}ms", delay);
        // 等待1秒
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    let mut delay_vec = delay_vec;
    delay_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // 计算网络延迟的正态分布参数、均值、方差
    let mut sum = 0.0;
    for delay in delay_vec.iter() {
        sum += delay;
    }
    let avg_delay = sum / (delay_vec.len()) as f64;
    let mut variance = 0.0;
    for delay in delay_vec.iter() {
        variance += (delay - avg_delay).powi(2);
    }
    variance /= (delay_vec.len()) as f64;
    let mut sum = 0.0;
    for delay in delay_vec.iter() {
        sum += (delay - avg_delay).powi(2);
    }
    let std_dev = (sum / (delay_vec.len()) as f64).sqrt();
    // 标准正态分布z值查询表
    // https://www.sjsu.edu/faculty/gerstman/EpiInfo/z-table.htm
    // -2.33对应的概率为0.9901
    let z = -2.33;
    let ret = avg_delay + z * std_dev;
    println!("均值: {:.4}, 方差: {:.4}, 99%概率> {:.4}ms", avg_delay, variance, ret as u64);
    return ret;
}

fn calculate_duration(target_time_str: &str) -> (Duration, chrono::DateTime<Local>) {
    let target_time: NaiveDateTime =
        NaiveDateTime::parse_from_str(target_time_str, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse NaiveDateTime");
    let date_time = Local.from_local_datetime(&target_time).unwrap();
    // 获取当前系统时间
    let now: chrono::DateTime<Local> = Local::now();
    println!("\n当前系统时间: {}", now);

    let timestamp_millis_now = now.timestamp_millis();
    let timestamp_millis_date_time = date_time.timestamp_millis();
    let delta_millis = timestamp_millis_date_time - timestamp_millis_now;
    if delta_millis < 0 {
        return (Duration::from_secs(0), now);
    }

    return (Duration::from_millis(delta_millis as u64), now);
}

pub fn wait_until(target_time: &str, ori_network_delay_ms: f64, use_delay: bool) {
    let mut offset_secs = sync();
    let mut network_delay_ms = ori_network_delay_ms;
    if !use_delay {
        network_delay_ms = 0.0;
    }
    let is_offset_negetive = offset_secs < 0.0;
    if is_offset_negetive {
        offset_secs = -offset_secs;
    }
    let offset_duration = Duration::from_secs_f64(offset_secs);
    let network_delay_duration = Duration::from_millis(network_delay_ms as u64);

    let (mut duration, from_local_time) = calculate_duration(target_time);
    println!("距离商品上架时长: {:.4}秒", duration.as_secs_f64());
    if duration.as_secs() == 0 {
        return;
    }

    if duration.lt(&offset_duration) {
        return;
    }
    if is_offset_negetive {
        duration = duration + offset_duration;
        println!(
            "同步阿里云服务器时间校正后等待时长(+{:.4}秒): {:.4}秒",
            offset_secs,
            duration.as_secs_f64()
        );
    } else {
        duration = duration - offset_duration;
        println!(
            "同步阿里云服务器时间校正后等待时长(-{:.4}秒): {:.4}秒",
            offset_secs,
            duration.as_secs_f64()
        );
    }

    if duration.lt(&network_delay_duration) {
        return;
    }
    duration = duration - network_delay_duration;
    println!(
        "考虑网络延迟后等待时长(-{:.4}毫秒): {:.4}秒",
        network_delay_ms,
        duration.as_secs_f64()
    );

    let adjust_start_time = from_local_time + duration;
    draw_flow(ori_network_delay_ms, adjust_start_time, use_delay);    

    let mut remaining_duration;
    let loading_vec = vec![
        "⠟",
        "⠯",
        "⠷",
        "⠾",
        "⠽",
        "⠻"];
    let mut loading_index = 0;
    loop {
        // 打印剩余时间
        let now = Local::now();
        remaining_duration = Duration::from_millis((adjust_start_time - now).num_milliseconds() as u64);
        if remaining_duration.as_millis() >= 3000 {
            // print!("\x1B[1A");
            Terminal::clear_current_line();
            print!(" {}剩余{:.0}秒\r", loading_vec.get(loading_index).unwrap(), remaining_duration.as_secs_f64());
            Terminal::flush().unwrap();
            loading_index = (loading_index + 1) % 6;
            sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }
    // print!("\x1B[1A");
    Terminal::clear_current_line();
    println!("即将结算...");
    Terminal::flush().unwrap();
    sleep(remaining_duration);

}

fn draw_flow(network_delay_ms: f64, adjust_start_time: chrono::DateTime<Local>, use_delay: bool) {
    /*
            商品上架时间点    结算完成                       提单完成
                 |            |                            |
                 |            |                            |
    ----*--32ms--*--checkout--*--32ms--*--32ms--*--submit--*
        |                              |
        |                              |
    发起结算请求                       发起提单请求
    10:59:59.968
     */
    let network_delay_ms_u64 = network_delay_ms as u64;
    let network_delay_ms_str = format!("{}", network_delay_ms_u64);
    let network_delay_ms_str_len = network_delay_ms_str.len();
    let space_str = " ".repeat(network_delay_ms_str_len);
    let border_str = "/".repeat(network_delay_ms_str_len);
    let adjust_start_time_str = adjust_start_time.format("%H:%M:%S%.3f").to_string();
    if use_delay {
        println!("\n/////////////////////////////////////////////////////////{}{}{}", border_str, border_str, border_str);
        println!("  {}  Product Launch    Checkout Done        {}{}   Submit Done", space_str, space_str, space_str);
        println!("       {}    |            |  {}       {}               |", space_str, space_str, space_str);
        println!("       {}    |            |  {}       {}               |", space_str, space_str, space_str);
        println!("----*--{}ms--*--checkout--*--{}ms--*--{}ms--*--submit--*", network_delay_ms_str, network_delay_ms_str, network_delay_ms_str);
        println!("    |  {}                    {}    |", space_str, space_str);
        println!("    |  {}                    {}    |", space_str, space_str);
        println!("Send Checkout Request     {}{}   Send Submit Request", space_str, space_str);
        println!("{} ", adjust_start_time_str);
        println!("(Local System Time)");
        println!("/////////////////////////////////////////////////////////{}{}{}\n", border_str, border_str, border_str);
    } else {
        println!("/////////////////////////////////////////////////////////{}{}{}", border_str, border_str, border_str);
        println!("Product Launch {}       Checkout Done        {}{}   Submit Done", space_str, space_str, space_str);
        println!("    |  {}                 |  {}       {}               |", space_str, space_str, space_str);
        println!("    |  {}                 |  {}       {}               |", space_str, space_str, space_str);
        println!("----*--{}ms--*--checkout--*--{}ms--*--{}ms--*--submit--*", network_delay_ms_str, network_delay_ms_str, network_delay_ms_str);
        println!("    |  {}                    {}    |", space_str, space_str);
        println!("    |  {}                    {}    |", space_str, space_str);
        println!("Send Checkout Request     {}{}   Send Submit Request", space_str, space_str);
        println!("{} ", adjust_start_time_str);
        println!("(Local System Time)");
        println!("/////////////////////////////////////////////////////////{}{}{}\n", border_str, border_str, border_str);
    }
}