use dotenv::dotenv;
use std::{
    env,
    error::Error,
};

use teloxide::{payloads::SendMessageSetters, prelude::*};
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
    #[command(description = "random choose amoung options")]
    Random(String),
    #[command(description = "shuffle options")]
    Shuffle(String),
    #[command(description = "analyze options")]
    Tell(String),
    #[command(description = "show a Binance sign up page")]
    Register,
    #[command(description = "show a cryptcurrency price in USDT by default")]
    Price(String),
}

async fn responses_to_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    match command {
        Command::Help | Command::Start
        => cx.answer(Command::descriptions()).send().await?, 

        Command::Random(option_list)
        => cx.answer(handler::misc::random::reply(option_list)).send().await?,

        Command::Shuffle(option_list)
        => cx.answer(handler::misc::shuffle::reply(option_list)).send().await?,

        Command::Tell(question_options)
        => cx.answer(handler::misc::tell::reply(question_options)).send().await?,

        Command::Register
        => cx.answer(handler::binance::register::reply()).parse_mode(MarkdownV2).send().await?,

        Command::Price(crpytocurrency)
        => cx.answer(handler::binance::price::reply(crpytocurrency)).send().await?,
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting the_bot...");

    let bot = Bot::from_env().auto_send();
    let bot_name: String = "binancie".into();

    teloxide::commands_repl(bot, bot_name, responses_to_command).await;
}