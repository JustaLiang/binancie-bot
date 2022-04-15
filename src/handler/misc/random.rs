use rand::Rng;

pub fn reply(option_list: String) -> String {
    let option_list = option_list.split_whitespace().collect::<Vec<&str>>();
    let option_size = option_list.len();
    match option_size {
        0 | 1 => "🤯".into(),
        _ => {
            let random_index = rand::thread_rng().gen_range(0..option_size);
            if let Some(&result) = option_list.get(random_index) {
                result.into()
            } else {
                "🤯".into()
            }
        }
    }
}