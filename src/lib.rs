use std::convert::Infallible;

use message_checks::bad_words;
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
            if let Some(video) = msg.video() {
                bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                    .await?;
                bot.send_message(msg.chat.id, "Video detected")
                    .reply_to_message_id(msg.id)
                    .await?;
                return Ok(());
            }

            if let Some(text) = msg.text() {
                let message = text.to_string().to_lowercase();
                if message_checks::url::is_url(&message) {
                    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
                        .await?;
                    bot.send_message(msg.chat.id, "URL detected")
                        .reply_to_message_id(msg.id)
                        .await?;
                    // return Ok(());
                }
                if bad_words::find_bad_words(&message).await {
                    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
                        .await?;
                    bot.send_message(msg.chat.id, "deficiente")
                        .reply_to_message_id(msg.id)
                        .await?;
                }
            }

            Ok(())
        },
        listener,
    )
    .await;
}
