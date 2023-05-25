// Good morning! Here's your coding interview problem for today.
//
// This problem was recently asked by Google.
//
// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
//
// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
//
// Bonus: Can you do this in one pass?

// Solution:
// 1. Place the values in a set space(n)
// 2. Loop over all elements in the array
// 3. Check if there is a value which adds up to k in the constructed set O(n) (Assuming look-up is
//    constant)
use std::collections::HashSet;

#[allow(dead_code)]
fn sum_of_numbers_in_array(arr: &[i32], k: i32) -> bool {
    let numbers: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());

    arr.iter().any(
        |&val| match (numbers.get(&(k - val)), numbers.get(&(val - k))) {
            (Some(_), _) => true,
            (_, Some(_)) => true,
            (None, None) => false,
        },
    )
}

// Test cases:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_positive_true() {
        assert!(sum_of_numbers_in_array(&[10, 15, 3, 7], 17));
    }

    #[test]
    fn test_only_positive_false() {
        assert!(!sum_of_numbers_in_array(&[10, 15, 3, 7], 100));
    }

    #[test]
    fn test_only_negative_true() {
        assert!(sum_of_numbers_in_array(&[-1, -5, -2, -10], -11))
    }

    #[test]
    fn test_only_negative_false() {
        assert!(!sum_of_numbers_in_array(&[-1, -5, -2, -10], 10))
    }

    #[test]
    fn test_mixed_true() {
        assert!(sum_of_numbers_in_array(&[-1, 2, -3, 4], 1));
    }

    #[test]
    fn test_mixed_false() {
        assert!(!sum_of_numbers_in_array(&[-1, 2, -3, 4], 10));
    }
}
