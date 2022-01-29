use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Commands {
    #[clap(
        short,
        long,
        multiple_values(true),
        value_names(&["Currency 1", "Currency 2"]),
        number_of_values(2),
        help("get the actual market price of the given currencies pair"),
    )]
    pub price: Option<Vec<String>>,
}