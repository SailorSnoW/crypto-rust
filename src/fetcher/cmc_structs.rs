use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CurrencyInfos {
    id: u32,
    name: String,
    symbol: String,
    category: String, // coin or token
    slug: String,
    logo: String,
    description: String,
    date_added: Option<String>,
    date_launched: Option<String>,
    notice: Option<String>,
    tags: Vec<String>,
    plateform: Option<Plateform>,
    self_reported_circulating_supply: Option<u128>,
    self_reported_market_cap: Option<u128>,
    self_reported_tags: Option<Vec<String>>,
    urls: Option<Urls>
}

impl fmt::Display for CurrencyInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.text_constructor()
        )
    }
}

impl CurrencyInfos {
    fn text_constructor(&self) -> String {
        let mut return_string = String::new();

        return_string.push_str(
           format!(
            "\nCurrency: {name} ({symbol})\n{name} is a {category}.",
            name = self.name,
            symbol = self.symbol,  
            category = self.category
           ).as_str()
        );

        match &self.date_launched {
            Some(date) => {
                return_string.push_str(
                    format!(
                     "\nWas launched the {}",
                     date
                    ).as_str()
                 );
            }
            None => ()
        }

        if self.tags.len() > 0 {
            return_string.push_str("\nTagged with: ");
            for (index, tag) in self.tags.iter().enumerate() {
                if index != 0 {
                    return_string.push_str(", ");
                }
                return_string.push_str(
                    format!(
                        "{}",
                        tag
                    ).as_str()
                )
            }
        }

        return return_string;
    }
}

#[derive(Deserialize, Debug)]
struct Plateform {
    id: u32,
    name: String,
    symbol: String,
    slug: String,
    token_address: String
}

#[derive(Deserialize, Debug)]
struct Urls {
    website: Vec<String>,
    technical_doc: Vec<String>,
    explorer: Vec<String>,
    source_code: Vec<String>,
    message_board: Vec<String>,
    chat: Vec<String>,
    announcement: Vec<String>,
    reddit: Vec<String>,
    twitter: Vec<String>
}