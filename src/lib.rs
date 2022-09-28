/// fib
/// ---
/// fib(0) = 0
/// fib(1) = 1
/// fib(n) = fib(n-1) + fib(n-2) for all n >= 0
pub fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}


#[cfg(test)]
mod tests {
    use crate::fib;

    #[test]
    fn test_fib_7() {
        assert_eq!(fib(7), 13);
    }

    #[test]
    fn test_fib_31() {
        assert_eq!(fib(31), 1346269);
    }
}
