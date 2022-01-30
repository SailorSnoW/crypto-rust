use std::{error::Error};
use serde_json::Value;

use crate::utils;

mod cmc_requests;

#[tokio::main(flavor = "current_thread")]
pub async fn get_pair_price(
    currency1: &String,
    currency2: &String,
    api_key: String
) -> Result<f64, Box<dyn Error>> {
    let mut parsed_currencies = String::new();
    parsed_currencies.push_str(currency1);
    parsed_currencies.push_str(",");
    parsed_currencies.push_str(currency2);


    let response = cmc_requests::get_currencies_market_data(parsed_currencies, api_key).await?;

    let response_jsonify: Value = serde_json::from_str(&response)?;

    let currency1_price_optionnal = &response_jsonify["data"][currency1]["quote"]["USD"]["price"].as_f64();
    let currency2_price_optionnal = &response_jsonify["data"][currency2]["quote"]["USD"]["price"].as_f64();

    return Ok(
        utils::compute_pair_price(
            currency1_price_optionnal.expect("Price error"),
            currency2_price_optionnal.expect("Price error"))
    )
}