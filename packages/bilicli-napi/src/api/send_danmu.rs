use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use serde::Deserialize;

#[derive(Deserialize)]
struct SendDanmuResponse {
    code: i32,
    message: String,
}

pub async fn send_danmu(room_id: u32, content: &str, cookie: String) -> Result<(), String> {
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());

    let kv: Vec<&str> = cookie.split("; ").collect::<Vec<&str>>();
    let cookie = kv.iter().find(|s| s.starts_with("bili_jct=")).unwrap();
    let cookie = cookie.split("=").nth(1).unwrap().to_string();
    eprintln!("content: {}", content);
    let mut params = HashMap::new();
    params.insert("csrf", cookie.to_string());
    params.insert("csrf_token", cookie.to_string());
    params.insert("color", "16777215".to_string());
    params.insert("fontsize", "25".to_string());
    params.insert("mode", "1".to_string());
    params.insert("msg", content.to_string());
    params.insert("rnd", "1724362289".to_string());
    params.insert("roomid", room_id.to_string());

    let client = reqwest::Client::new();

    let response = client
        .post("https://api.live.bilibili.com/msg/send")
        .headers(headers)
        .form(&params)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let body = response.json::<SendDanmuResponse>().await.unwrap();
        if body.code != 0 {
            return Err(body.message);
        }
        Ok(())
    } else {
        Err(response.text().await.unwrap())
    }
}
