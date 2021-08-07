use cgmath::{InnerSpace, Quaternion, Vector3, Zero};
use std::time::Duration;

pub struct KinematicProps {
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub max_acceleration: f32,
}

pub trait Kinematic {
    fn props(&self) -> KinematicProps;
    fn update(&mut self, steering: SteeringOutput, delta: Duration);
}

#[derive(Debug)]
pub struct SteeringOutput {
    pub linear: Vector3<f32>,
    pub angular: Vector3<f32>,
}

impl SteeringOutput {
    pub fn new() -> SteeringOutput {
        SteeringOutput {
            linear: Vector3::zero(),
            angular: Vector3::zero(),
        }
    }
}

pub fn combine(steering_behaviors: Vec<&SteeringOutput>) -> SteeringOutput {
    let mut result = SteeringOutput::new();

    for behavior in steering_behaviors {
        result.linear += behavior.linear;
        result.angular += behavior.angular;
    }

    result
}

pub fn stop(character_source: &impl Kinematic) -> SteeringOutput {
    let mut result = SteeringOutput::new();
    let character = character_source.props();

    // Gradual slowdown
    result.linear = -character.velocity;

    result
}

pub fn seek(character_source: &impl Kinematic, target_source: &impl Kinematic) -> SteeringOutput {
    let mut result = SteeringOutput::new();
    let character = character_source.props();
    let target = target_source.props();

    // Direction to target
    result.linear = target.position - character.position;

    // Get full acceleration along the direction
    result.linear.normalize();
    result.linear *= character.max_acceleration;

    result
}

pub fn flee(character_source: &impl Kinematic, target_source: &impl Kinematic) -> SteeringOutput {
    let mut result = SteeringOutput::new();
    let character = character_source.props();
    let target = target_source.props();

    // Direction away from target
    result.linear = character.position - target.position;

    // Get full acceleration along the direction
    result.linear.normalize();
    result.linear *= character.max_acceleration;

    result
}

pub fn rotate_by_position(target_source: &impl Kinematic) -> SteeringOutput {
    let mut result = SteeringOutput::new();
    let target = target_source.props();

    result.angular = target.position.normalize();

    result
}
