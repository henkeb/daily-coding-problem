// There's a staircase with N steps, and you can climb 1 or 2 steps at a time. Given N, write a function that returns the number of unique ways you can climb the staircase. The order of the steps matters.
// For example, if N is 4, then there are 5 unique ways:
//
//     1, 1, 1, 1
//     2, 1, 1
//     1, 2, 1
//     1, 1, 2
//     2, 2
//
// What if, instead of being able to climb 1 or 2 steps at a time, you could climb any number from a set of positive integers X? For example, if X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time. Generalize your function to take in X.
//

#[allow(dead_code)]
fn staircase(n: i32, x: &[i32]) -> i32 {
    match n {
        0 => 1,
        _ => x
            .iter()
            .filter(|&&i| i <= n)
            .map(|&i| staircase(n - i, x))
            .sum(),
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_input_1_2() {
        assert_eq!(staircase(1, &[1, 2]), 1);
        assert_eq!(staircase(2, &[1, 2]), 2);
        assert_eq!(staircase(3, &[1, 2]), 3);
        assert_eq!(staircase(4, &[1, 2]), 5);
    }

    #[test]
    fn ok_input_1_3() {
        assert_eq!(staircase(1, &[1, 3]), 1);
        assert_eq!(staircase(2, &[1, 3]), 1);
        assert_eq!(staircase(3, &[1, 3]), 2);
        assert_eq!(staircase(4, &[1, 3]), 3);
        assert_eq!(staircase(5, &[1, 3]), 4);
        assert_eq!(staircase(6, &[1, 3]), 6);
    }
}
