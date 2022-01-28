use std::error;

pub async fn get_currencies_market_data(
    currencies: String,
    api_key: String
) -> Result<String, Box<dyn error::Error>> {
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
    let params = [
        ("symbol", currencies),
        ("aux", String::from("total_supply"))
    ];

    let url_with_params = reqwest::Url::parse_with_params(url, params)?;

    let client = reqwest::Client::new();

    return Ok(
        client.get(url_with_params).header(
        "X-CMC_PRO_API_KEY", api_key
        ).send().await?.text().await?
    );
}