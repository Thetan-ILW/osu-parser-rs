// 'Who needs documentation?' module
pub fn to_bool(string: &str) -> bool {
    return match string.parse::<i8>() {
        Ok(value) => match value{
            0 => false,
            1 => true,
            _ => false
        },
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO BOOL: {string}");
            false
        }
    };
}

pub fn to_u8(string: &str) -> u8 {
    return match string.parse::<u8>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO u8: {string}");
            0
        }
    };
}

pub fn to_u32(string: &str) -> u32 {
    return match string.parse::<u32>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO u32: {string}");
            0
        }
    };
}

pub fn to_f32(string: &str) -> f32 {
    return match string.parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO f32: {string}");
            0.0
        }
    };
}

pub fn to_f64(string: &str) -> f64 {
    return match string.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO f32: {string}");
            0.0
        }
    };
}