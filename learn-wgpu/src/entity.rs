use crate::steering::{Kinematic, KinematicProps, SteeringOutput};

use cgmath::{prelude::*, Quaternion, Vector3, Zero};

pub struct Light {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub color: [f32; 3],
    pub chase_target_id: Option<u16>,
}

impl Light {
    pub fn new(position: Vector3<f32>, color: [f32; 3]) -> Light {
        Light {
            position,
            velocity: Vector3::zero(),
            color,
            chase_target_id: None,
        }
    }

    pub fn update_color(&mut self) {
        self.color = [
            distance_to_color_intensity(self.position.x),
            distance_to_color_intensity(self.position.y),
            distance_to_color_intensity(self.position.z),
        ];
    }
}

impl Kinematic for Light {
    fn props(&self) -> KinematicProps {
        KinematicProps {
            position: self.position,
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            velocity: self.velocity,
            rotation: cgmath::Vector3::zero(),
            max_acceleration: 1.5,
        }
    }

    fn update(&mut self, steering: SteeringOutput, delta: std::time::Duration) {
        let dt = delta.as_secs_f32();
        let max_speed = 0.95;

        // a Light can only move, not rotate
        self.position += self.velocity * dt;
        self.velocity = steering
            .linear
            .map_or(self.velocity, |linear| self.velocity + (linear * dt));

        if self.velocity.magnitude() > max_speed {
            self.velocity.normalize();
            self.velocity *= max_speed;
        }
    }
}

pub enum SpaceshipState {
    Fleeing,
    Wandering,
    Idle,
}

pub struct Spaceship {
    pub id: u16,
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub state: SpaceshipState,
}

impl Spaceship {
    pub fn new(id: u16, position: Vector3<f32>, orientation: Quaternion<f32>) -> Self {
        Spaceship {
            id,
            position,
            orientation,
            velocity: Vector3::zero(),
            rotation: Vector3::zero(),
            state: SpaceshipState::Idle,
        }
    }
}

impl Kinematic for Spaceship {
    fn props(&self) -> KinematicProps {
        KinematicProps {
            position: self.position,
            orientation: self.orientation,
            velocity: self.velocity,
            rotation: self.rotation,
            max_acceleration: 1.0,
        }
    }

    fn update(&mut self, steering: SteeringOutput, dt: std::time::Duration) {
        let dt = dt.as_secs_f32();
        let max_speed = 0.5;

        self.position += self.velocity * dt;
        self.orientation = self.orientation * delta_rotation(self.rotation, dt);
        self.velocity = steering
            .linear
            .map_or(Vector3::zero(), |linear| self.velocity + (linear * dt));
        self.rotation = steering
            .angular
            .map_or(Vector3::zero(), |angular| self.rotation + (angular * dt));

        if self.velocity.magnitude() > max_speed {
            self.velocity.normalize();
            self.velocity *= max_speed;
        }
    }
}

fn delta_rotation(rotation: Vector3<f32>, dt: f32) -> Quaternion<f32> {
    let half_angle_rotation_scaled = rotation * 0.5 * dt;
    let angle = half_angle_rotation_scaled.magnitude();

    // TODO: better names for the variables once I understand Quaternions well :)
    let (scalar_part, vector_part) = if angle > 0.0 {
        (
            angle.cos(),
            half_angle_rotation_scaled * angle.sin() / angle,
        )
    } else {
        (1.0, half_angle_rotation_scaled)
    };

    Quaternion::from_sv(scalar_part, vector_part)
}

fn distance_to_color_intensity(distance_from_origin: f32) -> f32 {
    let max_intensity_distance = 5.0;
    let ratio = distance_from_origin.abs() / max_intensity_distance;

    ratio.clamp(0.0, 1.0)
}
