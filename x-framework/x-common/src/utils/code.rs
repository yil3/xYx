use snowflake::SnowflakeIdGenerator;
use uuid::Uuid;

pub fn unique_id() -> String {
    let mut snow = SnowflakeIdGenerator::new(1, 1);
    let id = snow.real_time_generate() as u64;
    base_10_to_n(id, 36)
}

pub fn snow_id() -> i64 {
    let mut snow = SnowflakeIdGenerator::new(1, 1);
    snow.real_time_generate()
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

pub fn uuid_v4() -> String {
    Uuid::new_v4().to_string()
}

const ALL_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

/// 10 进制转 11 - 64 进制
/// ```
/// let id = 6888076346770202619;
/// assert_eq!(base_10_to_n(id, 36), "1gbyra5idyk8r");
/// ```
fn base_10_to_n(num: u64, radix: u32) -> String {
    if num == 0 {
        return String::from("0");
    }

    let base = base_10_to_n(num / (radix as u64), radix);
    let start = base.strip_prefix("0").unwrap_or(base.as_str());
    let end = match ALL_CHARS.chars().nth((num % (radix as u64)) as usize) {
        Some(data) => String::from(data),
        _ => String::from(""),
    };
    format!("{}{}", start, end)
}

/// 11 - 64 进制解析为 10 进制
/// ```
/// let id = "1gbyra5idyk8r";
/// assert_eq!(base_n_to_10(id, 36), 6888076346770202619);
/// ```
fn _base_n_to_10(num_str: &str, radix: u32) -> u128 {
    let mut result: u128 = 0;
    for i in 0..num_str.len() {
        result *= radix as u128;
        let target_char = num_str.chars().nth(i).unwrap_or('0');
        let data = ALL_CHARS.chars().position(|i| i == target_char).unwrap_or(0);
        result += data as u128;
    }
    result
}
