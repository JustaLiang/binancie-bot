use rand::Rng;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn reply() -> Option<InlineKeyboardMarkup> {
    let random_result = rand::thread_rng().gen_range(0..5);
    log::info!("{}", random_result);
    match random_result {
        0 => {
            let random_reward = rand::thread_rng().gen_range(1..=5);
            let callback_data = format!("reward:{}", random_reward);
            let keyboard: Vec<Vec<InlineKeyboardButton>> =
                vec![vec![InlineKeyboardButton::callback(
                    "claim".to_owned(),
                    callback_data.to_owned(),
                )]];
            Some(InlineKeyboardMarkup::new(keyboard))
        }
        _ => None,
    }
}
