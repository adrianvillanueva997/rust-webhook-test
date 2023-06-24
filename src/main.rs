use log::info;
use teloxide::{update_listeners::webhooks, Bot};
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot: Bot = Bot::from_env();
    let addr = ([127, 0, 0, 1], 8000).into();
    let url = "https://2053-90-94-189-135.eu.ngrok.io".parse().unwrap();
    // ngrok http 8000
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");
    info!("Running on {}", addr);
    webhookstuff::parse_messages(bot, listener).await;
}
