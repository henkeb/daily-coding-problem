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
fn staircase(step: i32) -> i32 {
    match step {
        0 => 1,
        1 => 1,
        _ => staircase(step - 1) + staircase(step - 2),
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_input() {
        assert_eq!(staircase(1), 1);
        assert_eq!(staircase(2), 2);
        assert_eq!(staircase(3), 3);
        assert_eq!(staircase(4), 5);
    }
}
