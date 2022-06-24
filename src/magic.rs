pub fn convert<T>(string: &str, default_value: T) 
    -> T where T: std::str::FromStr {
    return string.parse::<T>().unwrap_or_else(|_| {
        println!("Error: failed to convert value {string}"); 
        default_value
    });
}
