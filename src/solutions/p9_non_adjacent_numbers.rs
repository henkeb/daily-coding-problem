// Given a list of integers, write a function that returns the largest sum of non-adjacent numbers. Numbers can be 0 or negative.
//
// For example, [2, 4, 6, 2, 5] should return 13, since we pick 2, 6, and 5. [5, 1, 1, 5] should return 10, since we pick 5 and 5.

pub fn largest_sum_non_adj(input: &[usize]) -> Option<usize> {
    if input.is_empty() {
        return Some(0);
    }

    let first = *input.first().unwrap();

    if input.len() == 1 {
        return Some(first);
    }

    let mut including: usize = first;
    let mut excluding: usize = 0;

    for elem in input.iter().skip(1) {
        let exclude_next = including.max(excluding);
        match excluding.checked_add(*elem) {
            Some(val) => including = val,
            None => return None,
        }
        excluding = exclude_next;
    }
    Some(including.max(excluding))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_input() {
        assert_eq!(largest_sum_non_adj(&[2, 4, 6, 2, 5]), Some(13));
        assert_eq!(largest_sum_non_adj(&[5, 1, 1, 5]), Some(10));
        assert_eq!(largest_sum_non_adj(&[4, 1, 1, 4, 2, 1]), Some(9));
    }

    #[test]
    fn empty_input() {
        assert_eq!(largest_sum_non_adj(&[]), Some(0));
    }

    #[test]
    fn one_element_input() {
        assert_eq!(largest_sum_non_adj(&[1]), Some(1));
    }

    #[test]
    fn overflow() {
        assert_eq!(largest_sum_non_adj(&[usize::MAX, 1, 1]), None)
    }
}
