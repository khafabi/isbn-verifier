/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");

    if isbn.len() != 10 {
        return false;
    }

    let (first_nine, last_char) = (&isbn[..9], &isbn[9..]);

    if !first_nine.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    let last_val = match last_char.chars().next() {
        Some('X') => 10,
        Some(c) if c.is_digit(10) => c.to_digit(10).unwrap(),
        _ => return false,
    };

    let digits: Vec<u32> = first_nine
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let digits_with_last = [digits.as_slice(), &[last_val]].concat();

    let weight: Vec<u32> = (1..11).rev().collect();

    let sum = weighted_sum(&digits_with_last, &weight);

    sum % 11 == 0
}

fn weighted_sum(digits: &[u32], weights: &[u32]) -> u32 {
    if digits.is_empty() || weights.is_empty() {
        0
    } else {
        digits[0] * weights[0] + weighted_sum(&digits[1..], &weights[1..])
    }
}
