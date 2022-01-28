use std::io;
use std::fs;
use std::error;

use io::Read;
use io::Write;
use io::ErrorKind;

use crate::checker;
use checker::checker_error;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_key: "".into()
        }
    }
}



pub fn load_config() -> Result<Config, Box<dyn error::Error>> {
    let loaded_config = get_config();

    match loaded_config {
        Ok(config) => {
            match checker::verify_cmc_api_key_format(&config.api_key) {
                Ok(()) => return Ok(config),
                Err(e) => {
                    println!("{}", e);
                    return Err(e.into());
                }
            }
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    let config = read_config()?;
                    write_config(&config)?;
                    return load_config();
                }
                _ => return Err(e.into())
            }
        }
    }
}

fn read_config() -> Result<Config, Box<dyn error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Please enter your CMC API Key:");
    stdin.read_line(&mut buffer)?;

    match checker::verify_cmc_api_key_format(&buffer) {
        Ok(()) => return Ok(Config { api_key: buffer}),
        Err(e) => {
            match e {
                checker_error::Error::ApiKeyFormatError => {
                    println!("{}", e);
                    return Ok(Config::default());
                }
                _ => return Err(e.into())
            }
        }
    }
}

fn write_config(c: &Config) -> io::Result<()> {
    let mut file = fs::File::create("config.json")?;
    let serialized_config = serde_json::to_string_pretty(&c)?;

    file.write_all(serialized_config.as_bytes())?;

    return Ok(());
}

fn get_config() -> io::Result<Config> {
    let mut file = fs::File::open("config.json")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let deserialized_content = serde_json::from_str::<Config>(&content);

    match deserialized_content {
        Ok(config) => return Ok(config),
        Err(e) => {
            println!("Incorrect config file format, will re-generate the default config file.");
            write_config(&Config::default())?;
            return Err(e.into());
        }
    }
}