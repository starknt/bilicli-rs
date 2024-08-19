#![allow(dead_code)]

#[cfg(feature = "platform-napi")]
use napi_derive::napi;
use serde::Deserialize;
use strum::{Display, EnumIter, FromRepr};

pub mod footer;
pub mod header;
pub mod hyper_link;
pub mod tabs;

#[cfg(feature = "platform-napi")]
#[napi]
#[derive(Default, PartialEq, Eq, Debug)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
    Quit,
}

#[cfg(not(feature = "platform-napi"))]
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
    Quit,
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[cfg(feature = "platform-napi")]
#[napi]
#[derive(Debug, Display, FromRepr, EnumIter, PartialEq, Eq)]
pub enum MsgType {
    #[strum(to_string = "弹幕")]
    Danmu,
    #[strum(to_string = "SC")]
    SuperChat,
    #[strum(to_string = "礼物")]
    Gift,
    #[strum(to_string = "上舰")]
    GuardBuy,
    #[strum(to_string = "进场")]
    UserAction,
}

#[cfg(not(feature = "platform-napi"))]
#[derive(Debug, Clone, Copy, Display, FromRepr, EnumIter, PartialEq, Eq)]
pub enum MsgType {
    #[strum(to_string = "弹幕")]
    Danmu,
    #[strum(to_string = "SC")]
    SuperChat,
    #[strum(to_string = "礼物")]
    Gift,
    #[strum(to_string = "上舰")]
    GuardBuy,
    #[strum(to_string = "进场")]
    UserAction,
}

#[derive(Deserialize)]
pub struct Anchor {
    uid: u64,
    uname: String,
    room_id: u32,
    is_same_room: Option<bool>,
}

#[derive(Deserialize)]
pub struct Identity {
    rank: u8,
    guard_level: u8,
    room_admin: bool,
}

#[derive(Deserialize)]
pub struct Badge {
    // active: bool,
    name: String,
    level: u8,
    color: String,
    gradient: Option<Vec<String>>,
    anchor: Option<Anchor>,
    identity: Option<Identity>,
}

#[derive(Deserialize)]
pub struct User {
    uid: u64,
    uname: String,
    face: Option<String>,
    badge: Option<Badge>,
    identity: Option<Identity>,
}

#[derive(Deserialize)]
pub struct Emoticon {
    id: String,
    height: i32,
    width: i32,
    url: String,
}

#[derive(Deserialize)]
pub struct DanmuMsg {
    user: User,
    content: String,
    timestamp: i64,
    lottery: bool,
    emoticon: Option<Emoticon>,
}

#[derive(Deserialize)]
pub struct GuardBuyMsg {
    timestamp: i64,
    user: User,
    gift_id: u32,
    gift_name: String,
    guard_level: u8,
    price: u32,
    start_time: u32,
    end_time: u32,
}

#[derive(Deserialize)]
pub struct SuperChatMsg {
    id: u64,
    user: User,
    content: String,
    content_color: String,
    price: u32,
    time: u32,
    timestamp: i64,
}

#[derive(Deserialize)]
pub struct Master {
    uid: u64,
    uname: String,
    room_id: u32,
}

#[derive(Deserialize)]
pub struct Combo {
    batch_id: String,
    combo_num: u32,
    total_price: u32,
}

#[derive(Deserialize)]
pub struct GiftMsg {
    timestamp: i64,
    user: User,
    gift_id: u32,
    gift_name: String,
    coin_type: String,
    price: u32,
    amount: u32,
    send_master: Option<Master>,
    combo: Option<Combo>,
}

#[derive(Deserialize)]
pub struct UserActionMsg {
    user: User,
    action: String,
    timestamp: i64,
}
