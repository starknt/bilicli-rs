use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;

#[derive(Deserialize)]
struct SendDanmuResponse {
    code: i32,
    message: String,
    msg: String,
    ttl: i32,
}

pub async fn send_danmu(room_id: u64, content: &str) {
    let mut headers = HeaderMap::new();
    headers.append(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let body = format!("roomid={}&msg={}", room_id, content);

    let client = reqwest::Client::new();

    client
        .post("https://api.live.bilibili.com/msg/send")
        .headers(headers)
        .body(body)
        .send()
        .await
        .unwrap()
        .json::<SendDanmuResponse>()
        .await
        .unwrap();
}
