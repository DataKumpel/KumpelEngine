use hecs::World;
use glam::{Quat, Vec3};

use crate::components::{Transform, PointLight};


/// A system that rotates all transform around their y-axis.
pub fn rotate_cubes_system(world: &mut World, dt: f32) {
    for (transform,) in world.query_mut::<(&mut Transform,)>() {
        transform.rotation *= Quat::from_rotation_y(1.0 * dt);
    }
}

/// A system that circles light around the scene based on abs time.
pub fn animate_light_system(world: &mut World, total_time:  f32) -> (Vec3, Vec3) {
    let mut light_pos = Vec3::ZERO;
    let mut light_color = Vec3::ONE;

    for (transform, light) in world.query_mut::<(&mut Transform, &PointLight)>() {
        transform.position.x = total_time.cos() * 10.0;
        transform.position.z = total_time.sin() * 10.0;
        transform.position.y = 5.0 + (total_time * 2.0).sin() * 2.0;

        light_pos = transform.position;
        light_color = light.color;
    }

    (light_pos, light_color)
}
