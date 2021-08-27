use cgmath::prelude::*;
use cgmath::{InnerSpace, Quaternion, Rad, Vector3, Zero};
use rand::Rng;
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
    pub linear: Option<Vector3<f32>>,
    pub angular: Option<Vector3<f32>>,
}

impl SteeringOutput {
    pub fn new() -> SteeringOutput {
        SteeringOutput {
            linear: None,
            angular: None,
        }
    }
}

pub struct DummyKinematic {
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub max_acceleration: f32,
}

impl DummyKinematic {
    pub fn from_position(position: Vector3<f32>) -> Self {
        DummyKinematic {
            position,
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            velocity: Vector3::zero(),
            rotation: Vector3::zero(),
            max_acceleration: 0.0,
        }
    }

    pub fn from_orientation(orientation: Quaternion<f32>) -> Self {
        DummyKinematic {
            position: Vector3::zero(),
            orientation,
            velocity: Vector3::zero(),
            rotation: Vector3::zero(),
            max_acceleration: 0.0,
        }
    }
}

impl Kinematic for DummyKinematic {
    fn props(&self) -> KinematicProps {
        KinematicProps {
            position: self.position,
            orientation: self.orientation,
            velocity: self.velocity,
            rotation: self.rotation,
            max_acceleration: self.max_acceleration,
        }
    }

    fn update(&mut self, _steering: SteeringOutput, _delta: Duration) {}
}

// Mix steering behaviors

pub fn combine(steering_behaviors: Vec<&SteeringOutput>) -> SteeringOutput {
    let mut result = SteeringOutput::new();

    for behavior in steering_behaviors {
        result.linear = match (result.linear, behavior.linear) {
            (Some(a), Some(b)) => (Some(a + b)),
            _ => None,
        };
        result.angular = match (result.angular, behavior.angular) {
            (Some(a), Some(b)) => (Some(a + b)),
            _ => None,
        };
    }

    result
}

// Steering behaviors - linear

pub fn stop(character_source: &impl Kinematic) -> SteeringOutput {
    let mut result = SteeringOutput::new();
    let character = character_source.props();

    // Gradual slowdown
    result.linear = Some(-character.velocity);

    result
}

pub fn seek(character_source: &impl Kinematic, target_source: &impl Kinematic) -> SteeringOutput {
    let character = character_source.props();
    let target = target_source.props();

    // Direction to target
    let direction = (target.position - character.position).normalize();

    // Get full acceleration along the direction
    let result = direction * character.max_acceleration;

    SteeringOutput {
        linear: Some(result),
        angular: None,
    }
}

pub fn flee(character_source: &impl Kinematic, target_source: &impl Kinematic) -> SteeringOutput {
    let character = character_source.props();
    let target = target_source.props();

    // Direction away from target
    let direction = (character.position - target.position).normalize();

    // Get full acceleration along the direction
    let result = direction * character.max_acceleration;

    SteeringOutput {
        linear: Some(result),
        angular: None,
    }
}

// Steering behaviors - with angular component

// A minimal version of the align steering behavior for 2D rotation
fn align_2d(
    current_rotation: f32,
    current_orientation: Rad<f32>,
    target_orientation: Rad<f32>,
) -> Option<f32> {
    // Radians per second squared
    let max_angular_acceleration: f32 = 10.0;
    // Radians per second
    let max_rotation: f32 = 1.0;
    let target_radius: Rad<f32> = Rad(0.017);
    let slow_radius: Rad<f32> = Rad(1.0);
    // The time over which to achieve target speed
    let time_to_target: f32 = 0.1;

    let rotation = (target_orientation - current_orientation).normalize_signed();
    let rotation_size = Rad(rotation.0.abs());

    if rotation_size < target_radius {
        // No steering required
        println!("angle_2d: no rotation");
        return None;
    }

    println!("align_2d: rotation {:?}", rotation);
    println!("align_2d: rotation_size {:?}", rotation_size);

    let target_rotation: f32 = if rotation_size > slow_radius {
        max_rotation
    } else {
        max_rotation * rotation_size.0 / slow_radius.0
    };
    println!("align_2d: target_rotation {:?}", target_rotation);

    let target_rotation_with_direction: f32 = target_rotation * rotation.0 / rotation_size.0;
    println!(
        "align_2d: target_rotation_with_direction {:?}",
        target_rotation_with_direction
    );
    let angular_acceleration = (target_rotation_with_direction - current_rotation) / time_to_target;
    println!("align_2d: angular_acceleration {:?}", angular_acceleration);
    let result = angular_acceleration.clamp(-max_angular_acceleration, max_angular_acceleration);

    println!("align_2d: result {:?}", result);

    Some(result)
}

pub fn align(character_source: &impl Kinematic, target_source: &impl Kinematic) -> SteeringOutput {
    let character = character_source.props();
    let target = target_source.props();

    let target_orientation = (character.orientation.conjugate() * target.orientation).normalize();
    println!("align: target_orientation {:?}", target_orientation);
    let rotation_angle = Rad(quaternion_angle(target_orientation));
    println!("align: rotation_angle {:?}", rotation_angle);
    let rotation_axis = quaternion_axis(target_orientation);
    println!("align: rotation_axis {:?}", rotation_axis);

    let current_rotation = character.rotation.magnitude();
    println!("align: current_rotation {:?}", current_rotation);
    let align_result = align_2d(current_rotation, Rad(0.0), rotation_angle);

    let angular_output =
        align_result.map(|angular_acceleration| rotation_axis * angular_acceleration);

    SteeringOutput {
        linear: None,
        angular: angular_output,
    }
}

const BASE_ORIENTATION: Quaternion<f32> = Quaternion::new(1.0, 0.0, 0.0, 0.0);

pub fn face(character_source: &impl Kinematic, target_source: &impl Kinematic) -> SteeringOutput {
    let character = character_source.props();
    let target = target_source.props();
    let direction = target.position - character.position;

    println!("face: target.position {:?}", target.position);
    println!("face: direction {:?}", direction);

    if direction.magnitude() == 0.0 {
        println!("face: no steering");
        return SteeringOutput::new();
    }

    let target_orientation = face_direction(direction);

    align(
        character_source,
        &DummyKinematic::from_orientation(target_orientation),
    )
}

fn face_direction(direction: Vector3<f32>) -> Quaternion<f32> {
    let base_z_vector = BASE_ORIENTATION * Vector3::unit_z();
    let direction = direction.normalize();

    println!("face_direction: direction {:?}", direction);

    if base_z_vector == direction {
        println!("face_direction: base_orientation");
        BASE_ORIENTATION
    } else if base_z_vector == -direction {
        println!("face_direction: base_orientation inverse");
        BASE_ORIENTATION.conjugate()
    } else {
        // Find the minimum rotation to the target
        let axis = base_z_vector.cross(direction);
        println!("face_direction: axis {:?}", axis);

        // Numerical accuracy can sometimes cause a zero axis
        // default to base orientation to avoid a NaN Quaternion
        if axis == Vector3::zero() {
            return BASE_ORIENTATION;
        }

        let dot = base_z_vector.dot(direction);
        let angle = axis.magnitude().atan2(dot);
        println!("face_direction: angle {:?}", angle);

        Quaternion::from_axis_angle(axis.normalize(), Rad(angle))
    }
}

pub fn wander(character_source: &impl Kinematic) -> SteeringOutput {
    let wander_offset = Vector3::new(5.0, 0.0, 0.0);
    let wander_radius_xz = 2.0;
    let wander_radius_y = 2.0;
    // "should be strictly less than 1/sqrt(3) = 0.577 [...to avoid a zero length wander target]"
    let wander_rate = 0.5;
    let character = character_source.props();

    let wander_direction: Vector3<f32> = Vector3::new(
        random_binomial() * wander_rate,
        random_binomial() * wander_rate,
        random_binomial() * wander_rate,
    );

    wander_direction.normalize();

    // Calculate the transformed target direction and scale it
    let mut target = character.orientation * wander_direction;

    target.x *= wander_radius_xz;
    target.y *= wander_radius_y;
    target.z *= wander_radius_xz;
    // Offset the center of the wander circle
    // TODO: double check this
    target += character.position + (character.orientation * wander_offset);

    let face_output = face(character_source, &DummyKinematic::from_position(target));

    SteeringOutput {
        linear: Some(target.normalize() * 0.5),
        angular: face_output.angular,
    }
}

// Utility

fn random_binomial() -> f32 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0..1.0);
    let b = rng.gen_range(0.0..1.0);

    a - b
}

fn quaternion_angle(quaternion: Quaternion<f32>) -> f32 {
    2.0 * quaternion.s.acos()
}

fn quaternion_axis(quaternion: Quaternion<f32>) -> Vector3<f32> {
    let angle = quaternion_angle(quaternion);
    let sin_half_angle = (angle / 2.0).sin();
    let v = quaternion.v;

    if sin_half_angle < 0.001 {
        Vector3::unit_z()
    } else {
        Vector3::new(
            v.x / sin_half_angle,
            v.y / sin_half_angle,
            v.z / sin_half_angle,
        )
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::steering::{quaternion_angle, quaternion_axis};
    use cgmath::{prelude::*, Quaternion, Rad, Vector3};

    #[test]
    fn test_quaternion_axis_angle_extraction() {
        let axis = Vector3::new(0.0, 0.7, 0.1);
        let base_z_axis = Vector3::new(0.0, 0.0, 1.0);
        let large_angle = Rad(1.0);
        let small_angle = Rad(0.00005);
        let q_large_angle = Quaternion::from_axis_angle(axis, large_angle);
        let q_small_angle = Quaternion::from_axis_angle(axis, small_angle);

        assert_eq!(quaternion_angle(q_large_angle), large_angle.0);
        assert_eq!(quaternion_axis(q_large_angle), axis);

        assert_eq!(quaternion_angle(q_small_angle), 0.0);
        assert_eq!(quaternion_axis(q_small_angle), base_z_axis);
    }
}
