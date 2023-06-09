// Implement a job scheduler which takes in a function f and an integer n, and calls f after n milliseconds.
//
use std::{thread, time};

fn job_scheduler<F>(mut function: F, n: u64)
where
    F: FnMut(),
{
    thread::sleep(time::Duration::from_millis(n));
    function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_func() {
        let mut val = 0;
        let func = || val += 1;
        job_scheduler(func, 10);
        assert_eq!(val, 1);
    }
}
