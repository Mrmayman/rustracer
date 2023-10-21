use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

// Utility Functions

pub fn random_double(rng: &mut XorShiftRng) -> f64 {
    // Returns a random real in [0,1).
    rng.gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64, rng: &mut XorShiftRng) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_double(rng)
}

pub fn random_int(rng: &mut XorShiftRng) -> i32 {
    // Returns a random integer in [0, i32::MAX].
    let mut rng = XorShiftRng::from_seed([69; 16]);
    rng.gen::<i32>()
}

pub fn random_int_range(min: i32, max: i32, rng: &mut XorShiftRng) -> i32 {
    // Returns a random integer in [min, max).
    if min >= max {
        return min; // Return min if min is greater than or equal to max.
    }
    min + random_int(rng) % (max - min)
}

fn days_since_2000() -> f64 {
    todo!()
}

pub fn random_quick_cycle(seed: &mut i32) {
    let mut temp_num: i64 = *seed as i64;
    if temp_num == 0 {
        temp_num = (days_since_2000() * 86400000.0) as i64;
        temp_num = (((temp_num % 4294967311) * 44831) + 3099) % 4294967291;
        temp_num = ((temp_num * 1430713) + 1240931) % 4294967311;
        temp_num = (((temp_num * 29036779) + 60899) % 4294967291) - 2147483645;
    }
    if temp_num == -1812431566 {
        temp_num = 971880512;
        *seed = temp_num as i32;
        return;
    }
    temp_num = ((((temp_num + 2147483645) * 47471) % 4294967291) * 50000)
        + (((temp_num + 2147483645) * 4875) + 2158657949);
    temp_num = (temp_num % 4294967291) - 2147483645;
    if temp_num == 0 {
        temp_num = -1812431566;
    }
    *seed = temp_num as i32;
}
