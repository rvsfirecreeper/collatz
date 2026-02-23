pub struct CollatzResult {
    pub seed: u64,
    pub steps: u64,
}
pub fn collatz(seed: &u64) -> CollatzResult {
    let mut current = *seed;
    let mut steps: u64 = 0;
    while current != 1 {
        if current.is_multiple_of(2) {
            current >>= 1
        } else {
            current = current * 3 + 1
        }
        steps += 1;
    }
    CollatzResult { seed: *seed, steps }
}
