use crypto::digest::Digest;
use dotenv::dotenv;
use std::{
    env,
    error::Error,
    cmp::Ordering::Equal,
    time::SystemTime
};

use teloxide::{payloads::SendMessageSetters, prelude::*};
use teloxide::utils::command::BotCommand;
use teloxide::utils::markdown::link;
use teloxide::types::ParseMode::MarkdownV2;

use binance::api::*;
use binance::market::*;

use rand::prelude::*;

use crypto::sha3::Sha3;

const ERROR_REPLY: &str = "🤯";

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

fn get_hash_vector(string: &str) -> Vec<f32> {
    let mut hasher = Sha3::keccak256();
    let string = format!("{:?}", SystemTime::now()) + string;
    hasher.input_str(&string[..]);
    let mut byte_array: [u8; 13] = [0; 13];
    hasher.result(&mut byte_array);
    byte_array.iter().map(|&b| (b as f32) - 127.5).collect()
}

fn get_vector_length(vector: &Vec<f32>) -> f32 {
    vector.iter().map(|x| x*x).sum::<f32>().sqrt()
}

fn get_hash_distance(base: &str, target: &str) -> f32 {
    let base = get_hash_vector(base);
    let target = get_hash_vector(target);
    let inner_product: f32 = 
        base.iter().zip(target.iter()).map(|(x, y)| x * y).sum();
    inner_product
    / get_vector_length(&base)
    / get_vector_length(&target)
    / 2f32 + 0.5
}

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
    let market: Market = Binance::new(None, None);

    match command {
        Command::Help | Command::Start
        => cx.answer(Command::descriptions()).send().await?, 

        Command::Random(option_list)
        => {
            let option_list = option_list.split_whitespace().collect::<Vec<&str>>();
            let option_size = option_list.len();
            match option_size {
                0 | 1 => cx.answer(ERROR_REPLY).send().await?,
                _ => {
                    let random_index = rand::thread_rng().gen_range(0..option_size);
                    if let Some(&result) = option_list.get(random_index) {
                        cx.answer(result).send().await?
                    } else {
                        cx.answer(ERROR_REPLY).send().await?
                    }
                }
            }
        }

        Command::Shuffle(option_list)
        => {
            let mut option_list = option_list.split_whitespace().collect::<Vec<&str>>();
            let option_size = option_list.len();
            match option_size {
                0 | 1 => cx.answer(ERROR_REPLY).send().await?,
                _ => {
                    option_list.shuffle(&mut rand::thread_rng());
                    cx.answer(option_list.join(" ")).send().await?
                }
            }
        }

        Command::Tell(question_options)
        => {
            if let Some((question, option_list)) = question_options.split_once("?") {
                let option_list = option_list.split_whitespace().collect::<Vec<&str>>();
                let option_size = option_list.len();
                match option_size {
                    0 => cx.answer(ERROR_REPLY).send().await?,
                    _ => {
                        let mut opt_dist_pair: Vec<(&str, f32)> = option_list
                            .iter()
                            .map(|&opt|
                                (opt, get_hash_distance(question,opt))
                            ).collect();
                        opt_dist_pair.sort_by(
                            |&x, &y|
                            y.1.partial_cmp(&x.1).unwrap_or(Equal)
                        );
                        let result: String = opt_dist_pair
                            .iter()
                            .map(|&item|
                                format!("{} ({:.1}%)", item.0, item.1 * 100f32)
                            ).collect::<Vec<String>>().join("\n");
                        let result = format!("{}?\n{}", question, result);
                        cx.answer(result).send().await?
                    }
                } 
            } else {
                cx.answer(ERROR_REPLY).send().await?
            }
        }

        Command::Register
        => {
            let register_link = link("https://www.binance.com/en/activity/referral/offers/claim?ref=CPA_00WX9M3F3T", "Don't have a Binance account yet? You can register here\\.");

            cx.answer(register_link).parse_mode(MarkdownV2).send().await?
        },

        Command::Price(crpytocurrency)
        => {
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