// Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.
//
// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].
//
// Follow-up: what if you can't use division?
//
// Solution

#[allow(dead_code)]
fn product_of_elements_in_array(arr: &[i32]) -> Option<Vec<i32>> {
    let product_of_elements: Option<i32> = arr.iter().try_fold(1_i32, |acc, &x| acc.checked_mul(x));
    let mut output: Vec<i32> = Vec::new();

    match product_of_elements {
        None => return None,
        Some(0) => return None,
        Some(val) => arr.iter().for_each(|element| {
            output.push(val / element);
        }),
    }

    Some(output)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of_elements_in_array() {
        assert_eq!(
            product_of_elements_in_array(&[1, 2, 3, 4, 5]),
            Some(vec![120, 60, 40, 30, 24])
        );
        assert_eq!(
            product_of_elements_in_array(&[3, 2, 1]),
            Some(vec![2, 3, 6])
        );
    }

    #[test]
    fn test_zero_in_array() {
        assert_eq!(product_of_elements_in_array(&[0, 1, 2]), None);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(
            product_of_elements_in_array(&[3, 2, -1]),
            Some(vec![-2, -3, 6])
        )
    }

    #[test]
    fn test_overflow() {
        assert_eq!(product_of_elements_in_array(&[2, i32::MAX]), None);
    }
}
