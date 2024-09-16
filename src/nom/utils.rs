pub fn map_range(
    value: f64,
    input_min: f64,
    input_max: f64,
    output_min: f32,
    output_max: f32,
) -> f32 {
    ((value - input_min) / (input_max - input_min) * (output_max - output_min) as f64
        + output_min as f64) as f32
}

pub fn lerp_angle(start: f32, end: f32, speed: f32, delta_time: f32) -> f32 {
    // Normalize start and end angles to be within the range [0, 2π]
    let start = start.rem_euclid(2.0 * std::f32::consts::PI);
    let end = end.rem_euclid(2.0 * std::f32::consts::PI);

    // Calculate the delta between the angles
    let mut delta = end - start;

    // Normalize the delta to ensure the shortest rotation direction
    if delta > std::f32::consts::PI {
        delta -= 2.0 * std::f32::consts::PI;
    } else if delta < -std::f32::consts::PI {
        delta += 2.0 * std::f32::consts::PI;
    }

    // Cap the maximum angle change per frame to avoid spinning too fast
    let max_angle_change = speed * delta_time;
    if delta.abs() > max_angle_change {
        delta = delta.signum() * max_angle_change;
    }

    // Apply the delta to the current orientation
    let new_orientation = start + delta;

    // Normalize the new orientation to be within [0, 2π] radians
    new_orientation.rem_euclid(2.0 * std::f32::consts::PI)
}
