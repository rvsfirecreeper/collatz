struct result {
    seed: u64,
    steps: u64,
}
fn collatz(seed: &u64) -> result {
    let mut current = *seed;
    let mut steps: u64 = 0;
    let mut odd: u64;
    let mut even: u64;
    let mut oddmask: u64;
    let mut evenmask: u64;
    while current != 1 {
        odd = current * 3 + 1;
        even = current >> 1;
        oddmask = -(current & 1);
        evenmask = -(current & 1 ^ 1);
        current = (odd & oddmask) | (even & evenmask);
        steps += 1;
    }
    result {
        seed: *seed,
        steps: steps,
    }
}
