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
    let value = string.clone().parse::<T>();
    match value {
        Ok(value) => return value,
        Err(_) => println!("failed to convert value {string}")
    }

    return default_value
}