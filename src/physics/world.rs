use nalgebra::Vector2;
use nphysics2d::world::{MechanicalWorld, GeometricalWorld, DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle, DefaultBodySet, DefaultColliderSet, RigidBody};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::algebra::Inertia2;

pub struct PhysicsWorld {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    constraints: DefaultJointConstraintSet<f32>,
    forces: DefaultForceGeneratorSet<f32>,
}

impl PhysicsWorld {
    
    pub fn new() -> Self {
        let mechanical_world = DefaultMechanicalWorld::<f32>::new(Vector2::new(0.0, -9.81));
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let bodies = DefaultBodySet::<f32>::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let constraints = DefaultJointConstraintSet::<f32>::new();
        let forces = DefaultForceGeneratorSet::<f32>::new();

        PhysicsWorld {
            mechanical_world: mechanical_world,
            geometrical_world: geometrical_world,
            bodies: bodies,
            colliders: colliders,
            constraints: constraints,
            forces: forces,
        }
    }

    pub fn add(&mut self, rigid_body: RigidBody<f32>) -> DefaultBodyHandle {
        self.bodies.insert(rigid_body)
    }

    pub fn get_pos(&self, handle: DefaultBodyHandle) -> (f32, f32) {
        if let Some(body) = self.bodies.rigid_body(handle) {
            let position = body.position().translation.vector;

            (position.x, position.y)
        }
        else
        {
            (0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.constraints,
            &mut self.forces,
        )
    }
}