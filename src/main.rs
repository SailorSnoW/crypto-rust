use std::env;
use std::error;

use crypto_rust::utils;
use crypto_rust::fetcher;
use crypto_rust::{checker, config};

fn main() -> Result<(), Box<dyn error::Error>> {
    // Proccessing parameters (currencies)
    let parameters: Vec<String> = env::args().collect();
    let command: &String = &parameters[1];

    match command.as_str() {
        // print the list of available commands
        "help" => {
            println!("Available commands:\n price - get the market price of the specified cuurrencies pair")
        }
        // get the market price of a given pair
        "price" => {
            // parameters check
            let currencies = checker::params_for_price(parameters);
            match currencies {
                Ok(c) => pair_price(c)?,
                Err(e) => {
                    println!("{}", e);
                    return Err(e.into());
                }
            }
        }
        _ => {
            println!("Unknown command. Type '{} help' to get a list of commands.", env!("CARGO_PKG_NAME"));
        }
    }

    return Ok(());
}

pub fn pair_price(
    currencies: (String, String)
) -> Result<(), Box<dyn error::Error>> {
    // Proccessing api key (config file)
    let config: config::Config = config::load_config()?;

    // get the pair market price of 'first_currency'/'second_currency'
    let pair_price = fetcher::get_pair_price(
        &currencies.0,
        &currencies.1,
        config.api_key
    )?;

    println!(
        "Current {}/{} market price: {}$",
        currencies.0,
        currencies.1,
        utils::format_price(pair_price)
    );

    return Ok(());
}