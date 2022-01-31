use std::error;

use crate::config;

fn request(
    url: &str,
    params: &[(&str, String)]
) -> Result<String, Box<dyn error::Error>> {
    let api_key = config::load_config()?.api_key;

    let url_with_params = reqwest::Url::parse_with_params(url, params)?;

    let client = reqwest::blocking::Client::new();
    return Ok(
        client.get(url_with_params).header(
        "X-CMC_PRO_API_KEY", api_key
        ).send()?.text()?
    );
}

pub fn get_currencies_market_data(
    currencies: String,
) -> Result<String, Box<dyn error::Error>> {
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
    let params = [
        ("symbol", currencies),
        ("aux", String::from("total_supply"))
    ];

    return request(url, &params);
}

pub fn get_currencies_information(currencies: &Vec<String>) -> Result<String, Box<dyn error::Error>> {
    let mut currencies_parsed = String::new();

    for (index, currency) in currencies.iter().enumerate() {
        if index != 0 {
            currencies_parsed.push(',');
        }
        currencies_parsed.push_str(
            format!("{}", currency).as_str()
        );
        
    }

    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/info";
    let params = [
        ("symbol", currencies_parsed),
    ];

    return request(url, &params);
} 