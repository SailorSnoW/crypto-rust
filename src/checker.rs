use regex::Regex;
use checker_error::Error;

pub mod checker_error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {        
        #[error("Invalid currency format for \"{currency}\".\nCurrency should be only composed of letters with a length of 3 or 4 characters.\n")]
        CurrencyFormatError {
            currency: String
        },

        #[error("Invalid CMC api key format.\n\nCorrect format should be:\nXXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX\n")]
        ApiKeyFormatError
    }
}

pub fn verify_cmc_api_key_format(api_key: &String) -> Result<(), Error> {
    let api_key_regex = Regex::new(
        r"^[a-z0-9]{8}[-][a-z0-9]{4}[-][a-z0-9]{4}[-][a-z0-9]{4}[-][a-z0-9]{12}$"
    ).unwrap();

    if !api_key_regex.is_match(api_key) {
        eprintln!("{}", Error::ApiKeyFormatError.to_string());
        return Err(Error::ApiKeyFormatError);
    }
    else {
        return Ok(());
    }
}

pub fn verify_currency_format(currency: &String) -> Result<(), Error> {
    let currency_regex = Regex::new(r"^[A-Z]{3,4}$").unwrap();

    if !currency_regex.is_match(currency) {
        let throwed_error = Error::CurrencyFormatError{currency: String::from(currency)};
        eprintln!("{}", throwed_error.to_string());
        return Err(throwed_error);
    }
    else { 
        return Ok(());
    }
}