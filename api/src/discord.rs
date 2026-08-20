use serde::Serialize;
use spin_sdk::http::{Request, send};

pub(crate) struct DiscordNotifier {
    webhook_url: String,
    message: Option<String>,
}

impl DiscordNotifier {
    pub(crate) fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            message: None,
        }
    }

    pub(crate) fn with_message(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub(crate) async fn send(&self) -> anyhow::Result<()> {
        let payload = NotificationPayload {
            content: self
                .message
                .clone()
                .unwrap_or("Hello from Spin!".to_string()),
        };

        let payload = serde_json::to_string(&payload)?;

        let discord_request = Request::builder()
            .method("POST")
            .uri(&self.webhook_url)
            .header("Content-Type", "application/json")
            .body(payload)?;

        match send(discord_request).await {
            Ok(discord_response) => {
                let status = discord_response.status().as_u16();
                match status {
                    200 | 204 => Ok(()),
                    _ => Err(anyhow::anyhow!(
                        "Received {} from Discord",
                        discord_response.status()
                    )),
                }
            }
            Err(e) => Err(anyhow::anyhow!("Error sending notification: {:?}", e)),
        }
    }
}
#[derive(Serialize)]
pub struct NotificationPayload {
    pub content: String,
}
