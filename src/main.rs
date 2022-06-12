mod types;
use serde_json::from_str;
use tungstenite::{connect, Message};
use types::{Payload, WsMsg};
use url::Url;

const CHAT_ID: &str = "";
const TELEGRAM_BOT_TOKEN: &str = "";
const MASTODON_ACCESSS_TOKEN: &str = "";
const INSTANCE_URL: &str = "";
const HASHTAG: &str = "";

fn main() {
    let (mut socket, _) = connect(
        Url::parse(&format!(
            "wss://{}/api/v1/streaming?access_token={}&stream=hashtag&tag={}",
            INSTANCE_URL, MASTODON_ACCESSS_TOKEN, HASHTAG,
        ))
        .unwrap(),
    )
    .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading websocket");
        match msg {
            Message::Text(text) => {
                handle_message(&text);
            }
            _ => {}
        };
    }
}

fn handle_message(msg: &str) {
    let deserialized_ws_msg = from_str::<WsMsg>(msg).unwrap();
    if deserialized_ws_msg.event == "delete" {
        return;
    }
    let payload = from_str::<Payload>(&deserialized_ws_msg.payload).unwrap();
    send_to_telegram(payload);
}

fn send_to_telegram(payload: Payload) {
    let md = html2md::parse_html(&payload.content);
    minreq::Request::new(
        minreq::Method::Get,
        format!(
            "https://api.telegram.org/bot{}/sendMessage",
            TELEGRAM_BOT_TOKEN
        ),
    )
    .with_param("chat_id", CHAT_ID)
    .with_param("text", format!("{}: {}", payload.account.display_name, md))
    .with_param("parse_mode", "MarkdownV2")
    .send()
    .unwrap();
}
