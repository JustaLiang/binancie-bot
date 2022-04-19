use std::error::Error;
use teloxide::prelude2::*;

pub async fn handler(
    q: CallbackQuery,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(msg) = q.message {
        bot.edit_message_text(
            msg.chat.id,
            msg.id,
            format!("{} get reward", q.from.full_name()),
        ).await?;
    } else {
        log::error!("invalid callback query");
    }
    Ok(())
}