struct Interval {
    min: f32,
    max: f32,
}

fn interval_size(interval: Interval) -> f32 {
    return interval.max - interval.min;
}

fn interval_contains(interval: Interval, x: f32) -> bool {
    return interval.min <= x && x <= interval.max;
}

fn interval_surrounds(interval: Interval, x: f32) -> bool {
    return interval.min < x && x < interval.max;
}

fn interval_clamp(interval: Interval, x: f32) -> f32 {
    var ret = 0.0;
    if x < interval.min {
        ret = interval.min;
    } else if x > interval.max {
        ret = interval.max;
    } else {
        ret = x;
    }
    return ret;
}

const interval_empty = Interval(infinity, -pow(2.0, 126.0));
const interval_universe = Interval(-pow(2.0, 126.0), infinity);