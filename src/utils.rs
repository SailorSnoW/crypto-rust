pub fn format_price(price: f64) -> String {
    if price < 1.0 {
        return format!("{:.6}", price);
    }
    else {
        return format!("{:.1}", price);
    }
}

pub fn compute_pair_price(
    currency1_price: f64,
    currency2_price: f64
) -> f64 {
    return currency1_price / currency2_price;
}