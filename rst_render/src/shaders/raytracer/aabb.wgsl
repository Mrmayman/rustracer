struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

fn aabb_new(ax: f32, ay: f32, az: f32, bx: f32, by: f32, bz: f32) -> Aabb {
    Aabb {
        x: interval_new(ax, bx),
        y: interval_new(ay, by),
        z: interval_new(az, bz),
    }
}