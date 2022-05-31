pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64")
    }
    let two: u64 = 2;
    two.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, s| square(s) + acc)
}
