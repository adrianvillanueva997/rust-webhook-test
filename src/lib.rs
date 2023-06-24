use std::convert::Infallible;

use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::Message;
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;

mod message_checks;

pub async fn parse_messages(
    bot: Bot,
    listener: impl UpdateListener<Err = Infallible> + std::marker::Send,
) {
    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            if msg.text().is_none() {
                return Ok(());
            }
            let message_text = msg.text().unwrap_or_default();
            if message_checks::url::is_url(message_text) {
                bot.send_message(msg.chat.id, "URL detected")
                    .reply_to_message_id(msg.id)
                    .await?;
            }

            if msg.text().unwrap() == "ping" {
                bot.send_message(msg.chat.id, "pong")
                    .reply_to_message_id(msg.id)
                    .await?;
            }

            Ok(())
        },
        listener,
    )
    .await;
}
