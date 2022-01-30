use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Commands {
    #[clap(
        short,
        long,
        multiple_values(true),
        value_names(&["CURRENCY 1", "CURRENCY 2"]),
        number_of_values(2),
        help("get the actual market price of the given currencies pair"),
    )]
    pub price: Option<Vec<String>>,

    #[clap(
        short,
        long,
        multiple_values(true),
        value_name("CURRENCY"),
        help("get informations about the given currencies")
    )]
    pub info: Option<Vec<String>>,
}