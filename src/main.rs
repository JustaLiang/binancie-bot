use dotenv::dotenv;
use teloxide::dispatching2::UpdateFilterExt;
use std::error::Error;

use teloxide::prelude2::*;
use teloxide::utils::command::BotCommand;
use teloxide::types::ParseMode::MarkdownV2;

mod handler;

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

    // Binance
    #[command(description = "show a Binance sign up page")]
    Register,
    #[command(description = "show a cryptcurrency price in USDT by default")]
    Price(String),
}

async fn message_handler(
    msg: Message,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommand::parse(text, "binancie_bot") {
            Ok(Command::Help) |
            Ok(Command::Start)
            => bot.send_message(
                msg.chat.id,
                Command::descriptions()
            ).send().await?, 
    
            Ok(Command::Random(option_list))
            => bot.send_message(
                msg.chat.id,
                handler::misc::random::reply(option_list)
            ).send().await?,
    
            Ok(Command::Shuffle(option_list))
            => bot.send_message(
                msg.chat.id,
                handler::misc::shuffle::reply(option_list)
            ).send().await?,
    
            Ok(Command::Tell(question_options))
            => bot.send_message(
                msg.chat.id,
                handler::misc::tell::reply(question_options)
            ).send().await?,
    
            Ok(Command::Register)
            => bot.send_message(
                msg.chat.id,
                handler::binance::register::reply()
            ).parse_mode(MarkdownV2).send().await?,
    
            Ok(Command::Price(crpytocurrency))
            => bot.send_message(
                msg.chat.id,
                handler::binance::price::reply(crpytocurrency)
            ).send().await?,

            Err(_)
            => bot.send_message(
                msg.chat.id,
                "unsupported command"
            ).send().await?,
        };
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler).build().setup_ctrlc_handler().dispatch().await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}