use std::{
    cmp::Ordering::Equal,
    time::SystemTime,
};
use crypto::{
    digest::Digest,
    sha3::Sha3
};

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

pub fn reply(question_options: String) -> String {
    if let Some((question, option_list)) = question_options.split_once("?") {
        let option_list = option_list.split_whitespace().collect::<Vec<&str>>();
        let option_size = option_list.len();
        match option_size {
            0 => "🤯".into(),
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
                format!("{}?\n{}", question, result)
            }
        } 
    } else {
        "🤯".into()
    }
}