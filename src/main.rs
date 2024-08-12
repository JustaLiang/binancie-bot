use dotenv::dotenv;
use std::error::Error;
use teloxide::dispatching2::UpdateFilterExt;

use teloxide::prelude2::*;

mod callback;
mod message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message::handler))
        .branch(Update::filter_callback_query().endpoint(callback::handler));

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}
