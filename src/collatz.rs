#[derive(Copy, Clone)]
pub struct CollatzResult {
    pub seed: u64,
    pub steps: u64,
}
pub fn collatz(seed: u64) -> CollatzResult {
    let mut current = seed;
    let mut steps: u64 = 0;
    let mut tz: u32 = current.trailing_zeros();
    steps += tz as u64;
    current >>= tz;
    while current != 1 {
        current = current * 3 + 1;
        tz = current.trailing_zeros();
        steps += (1 + tz) as u64;
        current >>= tz;
    }
    CollatzResult { seed, steps }
}
