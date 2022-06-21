use std::collections::HashMap;

pub fn get_value<T>(data: &HashMap<String, String>, name: &str, default_value: T) 
    -> T where T: std::str::FromStr {
    match data.contains_key(name) {
        true => return convert(&data[name], default_value),
        false => {
            println!("key {name} does not exist");
            return default_value
        }
    }
}

pub fn convert<T>(string: &str, default_value: T) 
    -> T where T: std::str::FromStr {
    return string.parse::<T>().unwrap_or_else(|_| {
            println!("failed to convert value {string}"); 
            default_value
    });
}