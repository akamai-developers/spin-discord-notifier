use axum::Router;

use axum::http::StatusCode;
use axum::routing::post;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::{http_service, variables};
use tower_service::Service;

use crate::discord::DiscordNotifier;

const MESSAGES: [&str; 5] = [
    "We'll be live soon and build something amazing with Spin 💫!",
    "On Air soon! Get ready for an exciting journey with Spin 🚀!",
    "Huddle is starting in a few! Join us for some Spin magic ✨!",
    "Hey Wasm & Spin Friends 👋 We'll go live here on Discord soon 🎉",
    "Are you busy? Hopefully not, because we'll start streaming in a few 🎥",
];

mod discord;
#[http_service]
async fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Router::new()
        .route("/api/notifications", post(send_notification))
        .call(req)
        .await)
}

async fn send_notification() -> StatusCode {
    let random_message = MESSAGES[(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize)
        % MESSAGES.len()];
    let mut notifier = DiscordNotifier::new(
        variables::get("discord_webhook_url")
            .await
            .expect("Invalid App Config"),
    );
    match notifier
        .with_message(random_message.to_string())
        .send()
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Error sending notification: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
