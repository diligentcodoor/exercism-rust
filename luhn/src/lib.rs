/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let filtered_code = code.replace(" ", "");

    if filtered_code.len() <= 1 {
        return false;
    }

    if !filtered_code
        .chars()
        .all(|c| c.is_ascii() && c.is_numeric())
    {
        return false;
    }

    filtered_code
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            let digit = c.to_digit(10).unwrap();
            let digit_value = if i % 2 == 0 { digit } else { digit * 2 };
            acc + if digit_value > 9 {
                digit_value - 9
            } else {
                digit_value
            }
        })
        % 10
        == 0
}
