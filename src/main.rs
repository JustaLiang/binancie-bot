use dotenv::dotenv;
use std::{env, error::Error};

use teloxide::{payloads::SendMessageSetters, prelude::*};
use teloxide::utils::command::BotCommand;
use teloxide::utils::markdown::link;
use teloxide::types::ParseMode::MarkdownV2;

use binance::api::*;
use binance::market::*;

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

// Command examples
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "display this text.")]
  Help,
  #[command(description = "show a Binance sign up page.")]
  Register,
  #[command(description = "show a cryptcurrency price in USDT by default.")]
  Price(String),
}

async fn responses_to_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let market: Market = Binance::new(None, None);

    match command {
        // 1.
        Command::Help => cx.answer(Command::descriptions()).send().await?, 
        // 2.
        Command::Register => {
            let register_link = link("https://www.binance.com/en/activity/referral/offers/claim?ref=CPA_00WX9M3F3T", "Don't have a Binance account yet? You can register here\\.");

            cx.answer(register_link).parse_mode(MarkdownV2).send().await?
        },
        // 3.
        Command::Price(crpytocurrency) => {
            let mut iter = crpytocurrency.split_whitespace();

            if let Some(first_crypto_symbol) = iter.next() {

                let second_crypto_symbol = if let Some(second_crypto_symbol) = iter.next() {
                    second_crypto_symbol
                } else {
                    "USDT"
                };

                let target = to_uppercase(
                    &format!("{}{}", &first_crypto_symbol, &second_crypto_symbol)
                );

                match market.get_price(target) {
                    Ok(symbol_price) => {
                        cx.answer(format!("The price you want is {:#?}. ", &symbol_price.price)).await?
                    },
                    Err(e) => {
                        log::error!("{:#?}", e);
                        cx.answer(format!("Something went wrong. Did you use the correct cryptocurrency pair?")).await?
                    },
                }
            } else {
                cx.answer("Cryptocurrency symbols were not specified. To start with, you can use /price ETH or /price ETH USDT.").await?
            }
        }
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