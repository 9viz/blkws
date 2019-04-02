use std::time::{SystemTime, UNIX_EPOCH};

static mut RAND: u64 = 1;

unsafe fn gen_rand() {
    let a: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("I can't convert time for some reason")
        .as_secs();

    let c: u64 = 2;
    let m: u64 = 2_u64.pow(24) + RAND % 3;

    RAND = (a * RAND+ c) % m
}

pub unsafe fn rand_range(lower_limit: u64, upper_limit: u64) -> u64 {
    if RAND == 1 { for _ in 0..4 { gen_rand(); } }
    gen_rand();

    RAND % (upper_limit + 1 - lower_limit) + lower_limit
}
