pub fn string_to_color(input: &str) -> String {
    debug_assert!(input.len() <= 45);
    let hash = input
        .chars()
        .fold(0u128, |hash, char| (char as u128 + ((hash << 3) - hash)));

    (0..3).fold(String::new(), |mut color, i| {
        let value = (hash >> (i * 8)) & 0xff;
        let hex = format!("{value:X}");
        color.push_str(&format!("{hex:0<2}"));
        color
    })
}
