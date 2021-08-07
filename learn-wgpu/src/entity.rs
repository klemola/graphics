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
            position: self.position.into(),
            orientation: Quaternion::zero(),
            velocity: self.velocity,
            rotation: cgmath::Vector3::zero(),
            max_acceleration: 1.5,
        }
    }

    fn update(&mut self, steering: crate::steering::SteeringOutput, delta: std::time::Duration) {
        let dt = delta.as_secs_f32();
        let max_speed = 0.95;

        // a Light can only move, not rotate
        self.position += self.velocity * dt;
        self.velocity += steering.linear * dt;

        if self.velocity.magnitude() > max_speed {
            self.velocity.normalize();
            self.velocity *= max_speed;
        }
    }
}

pub enum CubeState {
    Fleeing,
    Wandering,
    Idle,
}

pub struct Cube {
    pub id: u16,
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub state: CubeState,
}

impl Cube {
    pub fn new(id: u16, position: Vector3<f32>, orientation: Quaternion<f32>) -> Self {
        Cube {
            id,
            position,
            orientation,
            velocity: Vector3::zero(),
            rotation: Vector3::zero(),
            state: CubeState::Idle,
        }
    }
}

impl Kinematic for Cube {
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

        let rotation = Quaternion::from_sv(1.0, self.rotation);
        self.orientation = self.orientation.normalize().slerp(rotation, dt);

        self.velocity += steering.linear * dt;
        self.rotation += steering.angular * dt;

        if self.velocity.magnitude() > max_speed {
            self.velocity.normalize();
            self.velocity *= max_speed;
        }
    }
}

fn distance_to_color_intensity(distance_from_origin: f32) -> f32 {
    let max_intensity_distance = 5.0;
    let ratio = distance_from_origin.abs() / max_intensity_distance;

    ratio.clamp(0.0, 1.0)
}
