pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let sum = digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum::<u32>();
    sum == num
}
