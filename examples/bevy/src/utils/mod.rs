use bevy::{
    camera::Camera,
    ecs::{
        observer::On,
        query::With,
        system::{Commands, Single},
    },
    math::Vec2,
    picking::events::{Drag, Pointer},
    transform::components::{GlobalTransform, Transform},
};

#[allow(dead_code)]
pub fn on_drag_and_drop(
    mut event: On<Pointer<Drag>>,
    mut commands: Commands,
    camera: Single<&GlobalTransform, With<Camera>>,
) {
    let Ok(mut entity_command) = commands.get_entity(event.entity) else {
        tracing::info!("Entity not found");
        return;
    };

    let transform = *camera;

    // flips y axis
    let flipped_delta = event.delta * (Vec2::X + Vec2::NEG_Y);

    // converts to world space
    let delta = transform
        .affine()
        .transform_vector3(flipped_delta.extend(0.0));

    event.propagate(false);

    entity_command
        .entry::<Transform>()
        .and_modify(move |mut transform| {
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        });
}
