// Given the mapping a = 1, b = 2, ... z = 26, and an encoded message, count the number of ways it can be decoded.
//
// For example, the message '111' would give 3, since it could be decoded as 'aaa', 'ka', and 'ak'.
//
// You can assume that the messages are decodable. For example, '001' is not allowed.
//
//
// Solution
//
// 1. Use dp with memoization.
//
// Solution takes O(n) in time and space

fn count_decoding(memo: &mut Vec<u8>, digits: &str, n: usize) -> usize {
    // Base case
    if n == 0 || n == 1 {
        return 1;
    }

    // If the digit starts with a '0' we should not count it
    if digits.starts_with('0') {
        return 0;
    }

    // If we have the value calculated, return it.
    if memo[n] != 0 {
        return std::convert::Into::<usize>::into(*memo.get(n).unwrap());
    }

    // Call recursively for 1 "used" char
    let mut count = count_decoding(memo, digits, n - 1);

    // Get start position of char and make sure that we still have more than 2 chars left
    // and also that the 2 chars are not more than 26
    // Call recursively for 2 "used" chars
    let s = digits.len() - n;
    if n >= 2 && digits[s..s + 2].parse::<usize>().unwrap() <= 26 {
        count += count_decoding(memo, digits, n - 2)
    }

    // Store in memo
    memo[n] = count as u8;

    count
}

fn count_ways(digits: &str) -> usize {
    let n = digits.len();
    if n == 0 {
        return 1;
    }

    let mut memo: Vec<u8> = vec![0; n + 1];

    count_decoding(&mut memo, digits, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let mut digits = "111";
        assert_eq!(count_ways(digits), 3);
        digits = "226";
        assert_eq!(count_ways(digits), 3);
        digits = "06";
        assert_eq!(count_ways(digits), 0);
    }
}
