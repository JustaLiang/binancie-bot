use std::error::Error;
use teloxide::prelude2::*;

pub async fn handler(
    q: CallbackQuery,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(msg) = q.message {
        if let Some(data) = q.data {
            match data.split(":").into_iter().next().unwrap_or("") {
                "reward" => {
                    bot.edit_message_text(
                        msg.chat.id,
                        msg.id,
                        format!("{} get reward", q.from.full_name()),
                    )
                    .await?;
                }
                _ => {}
            };
        } else {
            log::error!("invalid callback data");
        }
    } else {
        log::error!("invalid callback query");
    }
    Ok(())
}
