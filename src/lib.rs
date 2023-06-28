use std::convert::Infallible;

use log::error;
use message_checks::bad_words;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::Message;
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;

mod message_checks;

async fn process_text_messages(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(text) = msg.text() {
        let message = text.to_lowercase();
        let mut actions: Vec<_> = Vec::new();

        if message_checks::url::is_url(&message) {
            let twitter = message_checks::twitter::update_vxtwitter(&message).await;
            if !twitter.is_empty() {
                bot.delete_message(msg.chat.id, msg.id).await?;
                actions.push(
                    bot.send_message(msg.chat.id, twitter)
                        .reply_to_message_id(msg.id),
                );
            }
            actions.push(
                bot.send_message(msg.chat.id, "URL detected")
                    .reply_to_message_id(msg.id),
            );
        }

        if bad_words::find_bad_words(&message).await {
            actions.push(
                bot.send_message(msg.chat.id, "Deficiente")
                    .reply_to_message_id(msg.id),
            );
        }

        let copypastas = message_checks::copypasta::find_copypasta(&message);
        for copypasta in copypastas.await {
            actions.push(
                bot.send_message(msg.chat.id, copypasta)
                    .reply_to_message_id(msg.id),
            );
        }

        if !actions.is_empty() {
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
                .await?;
            tokio::join!(async {
                for action in actions {
                    action.await.unwrap();
                }
            });
        }
    }

    Ok(())
}

async fn process_video_messages(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(video) = msg.video() {
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await?;
        bot.send_message(msg.chat.id, "Video detected")
            .reply_to_message_id(msg.id)
            .await?;
    }
    Ok(())
}
pub async fn parse_messages(bot: Bot, listener: impl UpdateListener<Err = Infallible> + Send) {
    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            if let Err(err) = process_text_messages(bot.clone(), msg.clone()).await {
                error!("Error processing text messages: {}", err);
            }

            if let Err(err) = process_video_messages(bot.clone(), msg.clone()).await {
                error!("Error processing video messages: {}", err);
            }

            Ok(())
        },
        listener,
    )
    .await;
}
