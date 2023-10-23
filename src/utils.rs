use rand_xorshift::XorShiftRng;
use rand::Rng;

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
    rng.gen::<i32>()
}

pub fn random_int_range(min: i32, max: i32, rng: &mut XorShiftRng) -> i32 {
    // Ensure that min and max define a valid range
    if min >= max {
        return min;
    }

    // Generate a random integer within the range [min, max).
    // We're using the full range of i32 values here.
    let random_value = rng.gen::<i32>();
    let range = (max - min) as i64;
    let result = (random_value as i64 % range) + min as i64;

    // Ensure the result fits within the i32 range and return it.
    if result > i32::MAX as i64 {
        return i32::MAX;
    } else if result < i32::MIN as i64 {
        return i32::MIN;
    }

    result as i32
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}