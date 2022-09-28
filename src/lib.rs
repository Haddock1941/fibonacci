use std::collections::HashMap;

/// fib
/// ---
/// fib(0) = 0
/// fib(1) = 1
/// fib(n) = fib(n-1) + fib(n-2) for all n >= 0
///
/// * `cache` - A HashMap mapping (n => fib(n)) for caching results
pub fn fib(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    // tbh in real life I would use https://crates.io/crates/memoize
    // looks well maintained and there's rarely a reason to reinvent the wheel
    // but since this is a tech task and the point is probably to prove I've
    // understood what memoization even is, here you go:
    match cache.get(&n) {
        Some(cached_res) => *cached_res,
        None => {
            match n {
                0 => 0,
                1 => 1,
                n => {
                    let res = fib(n - 1, cache) + fib(n - 2, cache);
                    cache.insert(n, res);
                    res
                },
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::fib;

    use std::collections::HashMap;

    #[test]
    fn test_fib_7() {
        assert_eq!(fib(7, &mut HashMap::new()), 13);
    }

    #[test]
    fn test_fib_31() {
        assert_eq!(fib(31, &mut HashMap::new()), 1346269);
    }
}
