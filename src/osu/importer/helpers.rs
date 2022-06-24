use std::collections::{HashMap, BTreeMap};

pub fn convert<T>(string: &str, default_value: T) 
    -> T where T: std::str::FromStr {
    return string.parse::<T>().unwrap_or_else(|_| {
        println!("Error: failed to convert value {string}"); 
        default_value
    });
}

// works with numbers only
pub fn parse_and_set<T: std::str::FromStr>(value_data: &mut HashMap<&str, T>, section_data: &HashMap::<String,String>) {
    for (name, value) in value_data {
        let s = get_safely(section_data, name);
        let new_value = s.parse::<T>();
        match new_value {
            Ok(new_value) => *value = new_value,
            Err(_) => { 
                if s.len() != 0 {
                    println!("Error: failed to read {name}") 
                }
            }
        };
    }
}

// idk
pub fn parse_and_set_bool(value_data: &mut HashMap<&str, bool>, section_data: &HashMap::<String,String>) {
    for (name, value) in value_data {
        let s = get_safely(section_data, *name);
        match s {
            _ if s == "0" => *value = false,
            _ if s == "1" => *value = true,
            _ => { 
                if s.len() != 0 {
                    println!("Error: failed to read {name}") 
                }
            }
        };
    }
}

// Get the value from hashmap, if there is no or an error then return default value
// use only for strings
pub fn get_safely(data: &HashMap<String, String>, name: &str) -> String {
    match data.contains_key(name) {
        true => return data[name].clone(),
        false => {
            return String::new()
        }
    }
}

// Split [KEY:VALUE] and insert it to hashmap
pub fn get_key_value(section: &Vec<String>, data: &mut HashMap<String, String>) {
    for line in section {
        let key_value = line.split(":");
        let key_value = key_value.collect::<Vec<&str>>();
    
        if key_value.len() == 2 {
            data.insert(
                key_value[0].trim().to_string(), 
                key_value[1].trim().to_string()
            );
        }
    }
}

// hmmmmm, ш will come up with a solution tomorrow
pub fn get_key_value_btreemap(section: &Vec<String>, data: &mut BTreeMap<String, String>) {
    for line in section {
        let key_value = line.split(":");
        let key_value = key_value.collect::<Vec<&str>>();
    
        if key_value.len() == 2 {
            data.insert(
                key_value[0].trim().to_string(), 
                key_value[1].trim().to_string()
            );
        }
    }
}