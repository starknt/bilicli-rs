#![allow(unused)]

use std::collections::HashMap;

use chrono::{Local, Timelike};
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use serde::Deserialize;

#[derive(Deserialize)]
struct BaseUserInfo {
    name: String,
    face: String,
}

#[derive(Deserialize)]
struct User {
    uid: u64,
    base: BaseUserInfo,
}

#[derive(Deserialize)]
struct ModeInfo {
    mode: i32,
    user: User,
}

#[derive(Deserialize)]
struct SendDanmuData {
    mode_info: ModeInfo,
}

#[derive(Deserialize)]
struct SendDanmuResponse {
    code: i32,
    message: String,
    data: Option<SendDanmuData>,
}

pub async fn send_danmu(room_id: u32, content: &str, cookie: String) -> Result<(), String> {
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());

    let kv: Vec<&str> = cookie.split("; ").collect::<Vec<&str>>();
    let cookie = if let Some(cookie) = kv.iter().find(|s| s.starts_with("bili_jct=")) {
        if let Some(cookie) = cookie.split("=").nth(1) {
            cookie.to_string()
        } else {
            return Err("无法找到 csrf token".to_string());
        }
    } else {
        return Err("无法找到 csrf token".to_string());
    };

    let mut params = HashMap::new();
    params.insert("csrf", cookie.to_string());
    params.insert("csrf_token", cookie.to_string());
    params.insert("color", "16777215".to_string());
    params.insert("fontsize", "25".to_string());
    params.insert("mode", "1".to_string());
    params.insert("msg", content.to_string());
    params.insert("rnd", Local::now().timestamp().to_string());
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
