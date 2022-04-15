use binance::api::Binance;
use binance::market::Market;

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

pub fn reply(crpytocurrency: String) -> String {
    let market: Market = Binance::new(None, None);

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
                format!("The price you want is {:#?}. ", &symbol_price.price)
            },
            Err(e) => {
                log::error!("{:#?}", e);
                format!("Something went wrong. Did you use the correct cryptocurrency pair?")
            },
        }
    } else {
        "Cryptocurrency symbols were not specified. To start with, you can use /price ETH or /price ETH USDT.".into()
    }
}