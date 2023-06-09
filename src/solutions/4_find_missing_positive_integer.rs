// Given an array of integers, find the first missing positive integer in linear time and constant space. In other words, find the lowest positive integer that does not exist in the array. The array can contain duplicates and negative numbers as well.
//
// For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] should give 3.
//
// You can modify the input array in-place.
//
// This solution is using a set (which means that space is not constant).
// Time: O(n)
// Space: O(n)

fn find_missing_positive_integer(arr: &[i32]) -> i32 {
    let set: std::collections::HashSet<&i32> = arr.iter().collect();
    let mut output = 1;

    while set.contains(&output) {
        output += 1;
    }

    output
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_positive_integer() {
        assert_eq!(find_missing_positive_integer(&[3, 4, -1, 1]), 2);
        assert_eq!(find_missing_positive_integer(&[1, 2, 0]), 3);
    }
}
