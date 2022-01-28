use regex::Regex;
use checker_error::Error;

pub mod checker_error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Two currency parameters is required.\nSyntax: {} price <currency1> <currency2>\n", env!("CARGO_PKG_NAME"))]
        ParamCountError,
        
        #[error("Invalid currency format for \"{currency}\".\nCurrency should be only composed of letters with a length of 3 or 4 characters.")]
        CurrencyFormatError {
            currency: String
        },

        #[error("Invalid CMC api key format.\n\nCorrect format should be:\nXXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX\n")]
        ApiKeyFormatError
    }
}



pub fn params_for_price(
    args: Vec<String>
) -> Result<(String, String), checker_error::Error> {

    if args.len() < 4 {
        return Err(Error::ParamCountError);
    }

    let first_currency: String = args[2].to_uppercase();
    let second_currency: String = args[3].to_uppercase();

    verify_currency_format(&first_currency)?;
    verify_currency_format(&second_currency)?;

    return Ok((first_currency, second_currency));
}

pub fn verify_cmc_api_key_format(api_key: &String) -> Result<(), Error> {
    let api_key_regex = Regex::new(
        r"^[a-z0-9]{8}[-][a-z0-9]{4}[-][a-z0-9]{4}[-][a-z0-9]{4}[-][a-z0-9]{12}$"
    ).unwrap();

    if !api_key_regex.is_match(api_key) {
        return Err(Error::ApiKeyFormatError);
    }
    else {
        return Ok(());
    }
}

fn verify_currency_format(currency: &String) -> Result<(), Error> {
    let currency_regex = Regex::new(r"^[A-Z]{3,4}$").unwrap();

    if !currency_regex.is_match(currency) {
        return Err(Error::CurrencyFormatError{currency: String::from(currency)});
    }
    else { 
        return Ok(());
    }
}