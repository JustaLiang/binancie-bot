mod binance;
mod misc;

use std::error::Error;
use teloxide::prelude2::*;
use teloxide::utils::command::BotCommand;
use teloxide::types::ParseMode::MarkdownV2;

// Command examples
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "display this text")]
    Start,

    // Misc
    #[command(description = "random choose: [option1] [option2] ...")]
    Random(String),
    #[command(description = "shuffle items: [item1] [item2] ...")]
    Shuffle(String),
    #[command(description = "tell the answer: [question]? [option1] [option2] ...")]
    Tell(String),
    #[command(description = "convert to lowercase: [text]")]
    Lowercase(String),

    // Binance
    #[command(description = "show a Binance sign up page")]
    Register,
    #[command(description = "show a cryptcurrency price in USDT by default")]
    Price(String),
    #[command(description = "show an average price of a Binance symbol")]
    Average(String),
}

pub async fn handler(
    msg: Message,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommand::parse(text, "BinanceBot") {
            Ok(Command::Help) |
            Ok(Command::Start) => {
                bot.send_message(
                msg.chat.id,
                Command::descriptions()
                ).send().await?;
            }, 
    
            Ok(Command::Random(option_list)) => {
                bot.send_message(
                msg.chat.id,
                misc::random::reply(option_list)
                ).send().await?;
            },
    
            Ok(Command::Shuffle(option_list)) => {
                bot.send_message(
                msg.chat.id,
                misc::shuffle::reply(option_list)
                ).send().await?;
            },
    
            Ok(Command::Tell(question_options)) => {
                bot.send_message(
                msg.chat.id,
                misc::tell::reply(question_options)
                ).send().await?;
            },

            Ok(Command::Lowercase(text)) => {
                bot.send_message(
                msg.chat.id,
                misc::lowercase::reply(text)
                ).send().await?;
            },

            Ok(Command::Register) => {
                bot.send_message(
                msg.chat.id,
                binance::register::reply()
                ).parse_mode(MarkdownV2).send().await?;
            },
    
            Ok(Command::Price(crpytocurrency)) => {
                bot.send_message(
                msg.chat.id,
                binance::price::reply(crpytocurrency)
                ).send().await?;
            },

            Ok(Command::Average(crpytocurrency)) => {
                bot.send_message(
                msg.chat.id,
                binance::average::reply(crpytocurrency)
                ).send().await?;
            },

            Err(_) => {
                match misc::default::reply() {
                    Some(button) => {
                        bot
                        .send_message(msg.chat.id, "mining reward")
                        .reply_markup(button).await?;
                    },
                    None => {},
                };
            }
        };
    }

    Ok(())
}
