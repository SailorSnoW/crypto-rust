use std::error;

use clap::StructOpt;

use crypto_rust::{
    checker,
    config,
    fetcher,
    utils,
    commands::Commands
};

fn main() -> Result<(), Box<dyn error::Error>> {
    human_panic::setup_panic!();

    let args = Commands::parse();

    match args.price {
        Some(currencies) => {
            let currencies_upped = currencies.into_iter().map(|c| {
                let c_upped = c.to_uppercase();
                match checker::verify_currency_format(&c_upped) {
                    Ok(()) => return Ok(c_upped),
                    Err(e) => return Err(e)
                }
            }).collect::<Result<Vec<String>, checker::checker_error::Error>>()?;

            checker::verify_currency_format(&currencies_upped[0])?;
            checker::verify_currency_format(&currencies_upped[1])?;

            price(currencies_upped)?;   
            return Ok(()); 
        },
        None => ()
    }

    match args.info {
        Some(currencies) => {
            let currencies_upped = currencies.into_iter().map(|c| {
                let c_upped = c.to_uppercase();
                match checker::verify_currency_format(&c_upped) {
                    Ok(()) => return Ok(c_upped),
                    Err(e) => return Err(e)
                }
            }).collect::<Result<Vec<String>, checker::checker_error::Error>>()?;

            println!("currencies: {:?}", currencies_upped);
        },
        None => ()
    }
    
    return Ok(());
}

pub fn price(
    currencies: Vec<String>
) -> Result<(), Box<dyn error::Error>> {
    // Proccessing api key (config file)
    let config: config::Config = config::load_config()?;

    // get the pair market price of 'first_currency'/'second_currency'
    let pair_price = fetcher::get_pair_price(
        &currencies[0],
        &currencies[1],
        config.api_key
    )?;

    println!(
        "Current {}/{} market price: {}$",
        currencies[0],
        currencies[1],
        utils::format_price(pair_price)
    );

    return Ok(());
}

pub fn informations() -> Result<(), Box<dyn error::Error>> {
    // Proccessing api key (config file)
    let config: config::Config = config::load_config()?;

    return Ok(());
}