use nalgebra::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, RigidBody};
use nphysics2d::math::{Velocity, Inertia};

pub fn create_rbody() -> RigidBody<f32> {
    let rigid_body = RigidBodyDesc::new()
        .mass(50.0)
        .build();

    rigid_body
}