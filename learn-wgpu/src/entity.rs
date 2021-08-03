use crate::steering::{Kinematic, KinematicProps, SteeringOutput};

use cgmath::{prelude::*, Quaternion, Vector3, Zero};

pub struct Light {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub color: [f32; 3],
}

impl Light {
    pub fn new(position: Vector3<f32>, color: [f32; 3]) -> Light {
        Light {
            position,
            velocity: Vector3::zero(),
            color,
        }
    }
}

impl Kinematic for Light {
    fn props(&self) -> KinematicProps {
        KinematicProps::new(self.position.into(), Quaternion::zero())
    }

    fn update(&mut self, steering: crate::steering::SteeringOutput, delta: std::time::Duration) {
        let dt = delta.as_secs_f32();
        let max_speed = 0.8;

        // a Light can only move, not rotate
        self.position += self.velocity * dt;
        self.velocity += steering.linear * dt;

        if self.velocity.magnitude() > max_speed {
            self.velocity.normalize();
            self.velocity *= max_speed;
        }
    }
}

pub struct Cube {
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Kinematic for Cube {
    fn props(&self) -> KinematicProps {
        KinematicProps {
            position: self.position,
            orientation: self.orientation,
            velocity: self.velocity,
            rotation: self.rotation,
            max_acceleration: 0.5,
        }
    }

    fn update(&mut self, steering: SteeringOutput, dt: std::time::Duration) {
        let dt = dt.as_secs_f32();
        let max_speed = 0.6;

        self.position += self.velocity * dt;

        // TODO: check if this works
        let rotation = Quaternion::from_sv(1.0, self.rotation);
        self.orientation = self.orientation.lerp(rotation, dt);

        self.velocity += steering.linear * dt;
        self.rotation += steering.angular * dt;

        if self.velocity.magnitude() > max_speed {
            self.velocity.normalize();
            self.velocity *= max_speed;
        }
    }
}
