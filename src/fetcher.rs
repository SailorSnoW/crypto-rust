use std::error::Error;
use serde_json::Value;

use crate::utils;

mod cmc_requests;
mod cmc_structs;

pub fn pair_price(
    currency1: &String,
    currency2: &String,
) -> Result<f64, Box<dyn Error>> {
    let mut parsed_currencies = String::new();
    parsed_currencies.push_str(currency1);
    parsed_currencies.push_str(",");
    parsed_currencies.push_str(currency2);


    let response_raw = cmc_requests::get_currencies_market_data(parsed_currencies)?;

    let response_jsonify: Value = serde_json::from_str(&response_raw)?;

    let currency1_price_optionnal = &response_jsonify["data"][currency1]["quote"]["USD"]["price"].as_f64();
    let currency2_price_optionnal = &response_jsonify["data"][currency2]["quote"]["USD"]["price"].as_f64();

    return Ok(
        utils::compute_pair_price(
            currency1_price_optionnal.expect("Price error"),
            currency2_price_optionnal.expect("Price error"))
    )
}

pub fn currencies_information(currencies: Vec<String>) -> Result<Vec<cmc_structs::CurrencyInfos>, Box<dyn Error>> {
    let response_raw = cmc_requests::get_currencies_information(&currencies)?;

    let response_jsonify: Value = serde_json::from_str(&response_raw)?;
    let informations = response_jsonify["data"].as_object().unwrap();
    
    return Ok(currencies.into_iter().map(
        |currency_name| {
            let currency_informations = informations[&currency_name].to_string();
            return serde_json::from_str(&currency_informations).unwrap();
        }
    ).collect::<Vec<cmc_structs::CurrencyInfos>>());
}