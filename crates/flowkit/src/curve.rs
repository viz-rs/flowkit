use glam::Vec2;

/// Calculates the control offset for a curve based on the distance and curvature.
#[inline]
pub fn calculate_control_offset(distance: f32, curvature: f32) -> f32 {
    if distance == 0.0 {
        return 0.0;
    }

    let delta = distance.abs();

    // uses a smooth control offset that scales appropriately
    (delta.sqrt() * curvature)
        .min(delta * 0.5) // doesn't exceed half the distance
        .max(curvature) // maintains minimum offset for visual appeal
}

/// Calculates the control point for a curve based on the source and target points, edge position, curvature, and offset.
#[inline]
pub fn calculate_control_point(
    source_pos: Vec2,
    target_pos: Vec2,
    direction: Vec2, // Its an `EdgeAnchor`
    curvature: f32,
    offset: f32,
) -> Vec2 {
    let delta = source_pos - target_pos;
    let factor = curvature * offset;

    let x = calculate_control_offset(delta.x, factor);
    let y = calculate_control_offset(delta.y, factor);

    source_pos + direction * Vec2::new(x, y)
}
