# Discord Notifier

This app is a lightweight Spin application that serves a small frontend and an API endpoint for sending notifications. It posts random messages to a Discord channel through a configured webhook so you can quickly demo or trigger community updates from a Spin app.

## Prerequisites

- Node.js
- Rust
- Spin

## Setting up a Discord webhook

1. Open your Discord server and go to Server Settings > Integrations > Webhooks.
2. Click New Webhook, choose the channel to post to, and copy the generated webhook URL.
3. Keep this URL secret because it allows anyone with it to send messages to that Discord channel.

## Running the app

From the project root, start the app with Spin and pass the webhook URL as a variable:

```bash
spin up --build \
  --variable discord_webhook_url="https://discord.com/api/webhooks/your-webhook-id/your-token"
```

This will build both, the frontend and the API component and serve them on your local machine using port `3000`.
