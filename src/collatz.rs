#[derive(Copy, Clone)]
pub struct CollatzResult {
    pub seed: u64,
    pub steps: u64,
}
pub fn collatz(seed: &u64) -> CollatzResult {
    let mut current = *seed;
    let mut steps: u64 = 0;
    while current != 1 {
        if !current.is_multiple_of(2) {
            current = current * 3 + 1;
            steps += (1 + current.trailing_zeros()) as u64;
            current >>= current.trailing_zeros();
        } else {
            steps += current.trailing_zeros() as u64;
            current >>= current.trailing_zeros();
        }
    }
    CollatzResult { seed: *seed, steps }
}
