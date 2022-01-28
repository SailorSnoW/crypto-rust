pub fn format_price(price: f64) -> String {
    if price < 1.0 {
        return format!("{:.6}", price);
    }
    else {
        return format!("{:.1}", price);
    }
}