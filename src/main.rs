use log::info;
use std::{env, str::FromStr};
use teloxide::{update_listeners::webhooks, Bot};
use url::Url;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot: Bot = Bot::from_env();
    let addr = ([127, 0, 0, 1], 80).into();
    let url = env::var("url").expect("URL is not set");
    let url = Url::from_str(&url).unwrap();
    // let url = "https://84c1-90-94-189-135.eu.ngrok.io".parse().unwrap();
    // ngrok http 8000
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");
    info!("Running on {}", addr);
    webhookstuff::parse_messages(bot, listener).await;
}
