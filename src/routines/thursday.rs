use std::time::Duration;

use log::info;
use teloxide::{requests::Requester, types::ChatId, Bot};
use tokio::time;

use crate::routines::utils::{calculate_next_midnight, is_thursday, GROUP_ID};

pub async fn happy_thursday_routine(bot: Bot) {
    info!("Starting thursday routine");
    let mut sent = false;

    loop {
        if is_thursday() && !sent {
            info!("Running thursday routine");
            let _ = bot.send_message(ChatId(GROUP_ID), "Feliz jueves!").await;
            sent = true;
        }

        info!("Calculating next midnight");
        let sleep_seconds = calculate_next_midnight();
        time::sleep(Duration::from_secs(sleep_seconds)).await;

        if is_thursday() && sent {
            sent = false;
        }
    }
}
