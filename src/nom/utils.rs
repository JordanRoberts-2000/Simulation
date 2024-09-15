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
    let mut delta = end - start;

    // Ensure the shortest rotation direction in degrees
    if delta > 180.0 {
        delta -= 360.0;
    } else if delta < -180.0 {
        delta += 360.0;
    }

    // Cap the maximum angle change per frame to avoid spinning too fast
    let max_angle_change = speed * delta_time;
    if delta.abs() > max_angle_change {
        delta = delta.signum() * max_angle_change;
    }

    // Apply the delta to the current orientation
    let new_orientation = start + delta;

    // Wrap the result between 0 and 360 degrees to prevent accumulation errors
    new_orientation.rem_euclid(360.0)
}
