use rand::prelude::*;

pub fn reply(option_list: String) -> String {
    let mut option_list = option_list.split_whitespace().collect::<Vec<&str>>();
    let option_size = option_list.len();
    match option_size {
        0 | 1 => "🤯".into(),
        _ => {
            option_list.shuffle(&mut rand::thread_rng());
            option_list.join(" ")
        }
    }
}