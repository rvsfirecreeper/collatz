pub struct CollatzResult {
    pub seed: u64,
    pub steps: u64,
}
pub fn collatz(seed: &u64) -> CollatzResult {
    let mut current = *seed;
    let mut steps: u64 = 0;
    let mut odd: u64;
    let mut even: u64;
    let mut oddmask: u64;
    let mut evenmask: u64;
    while current != 1 {
        odd = current * 3 + 1;
        even = current >> 1;
        oddmask = (!0u64) * (current & 1);
        evenmask = (!0u64) * (current & 1 ^ 1);
        current = (odd & oddmask) | (even & evenmask);
        steps += 1;
    }
    CollatzResult {
        seed: *seed,
        steps: steps,
    }
}
