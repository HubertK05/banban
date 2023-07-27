use crate::errors::AppError;

const PRIME_MUL: i64 = 2147462143;
const PRIME_MOD: i64 = 998244353;

pub fn string_to_color(input: &str) -> i32 {
    let hash = input.chars().fold(0i64, |hash, char| {
        (char as i64 * PRIME_MUL + ((hash << 8) - hash)) % PRIME_MOD
    });

    (hash & 0xffffff) as i32
}

pub fn int_to_rgb(input: i32) -> (i32, i32, i32) {
    (input & 0xff0000, input & 0xff00, input & 0xff)
}

pub fn rgb_string_to_int(input: &str) -> Result<i32, AppError> {
    i32::from_str_radix(input, 16).map_err(|_| AppError::InvalidColor)
}

pub fn rgb_int_to_string(input: i32) -> String {
    (0..3).fold(String::from("#"), |mut color, i| {
        let value = (input >> ((2 - i) * 8)) & 0xff;
        let hex = format!("{value:X}");
        color.push_str(&format!("{hex:0<2}"));
        color
    })
}
