use bevy::prelude::*;
use std::f32;
use crate::lib::BoidGroup;
use rand::Rng;


#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Boid {
    pub position: Vec3,
    pub direction: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_force: f32,
    pub max_speed: f32,
    pub floc_radius: f32,
    pub local_radius: f32,
}

impl Boid {
    pub fn new(position: Vec3, direction: f32, velocity: Vec2) -> Self {
        Self {
            position,
            direction,
            velocity: Vec2::new(10.10, 10.10),
            acceleration: Vec2::new(0.0, 0.0),
            max_force: 0.1,
            max_speed: 20.0,
            floc_radius: 30.0,
            local_radius: 50.0,
            
        }
    }

    pub fn update(&mut self, boid_group: BoidGroup) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(0.0, self.max_speed);
        let new_y = f32::sin(self.direction.to_radians()) * self.velocity.y;
        let new_x = f32::cos(self.direction.to_radians()) * self.velocity.x;

        self.position.x += new_x;
        self.position.y += new_y;

        self.align(&boid_group);
        self.separation(&boid_group);
        self.cohersion(&boid_group);

        self.acceleration *= 0.0;
    }

    pub fn align (&mut self, boid_group: &BoidGroup) {
        let mut average_rotation = 0.0;
        let mut count = 0;
        for boid in boid_group.boids.iter() {
            let distance = self.distance_between(*boid);
            if boid != self && distance < self.floc_radius {
                average_rotation += boid.direction;
                count += 1;
            }
        }
        if count > 0 {
            average_rotation /= count as f32;
            let steer = average_rotation - self.direction;
            self.apply_force(Vec2::new(steer, steer));
        }
    }

    pub fn separation(&mut self, boid_group: &BoidGroup) {
        let mut average_position = Vec3::new(0.0, 0.0, 0.0);
        let mut count = 0;
        for boid in boid_group.boids.iter() {
            let distance = self.distance_between(*boid);
            if boid != self && distance < self.local_radius {
                let diff = self.position - boid.position;
                average_position += diff;
                count += 1;
            }
        }
        if count > 0 {
            average_position /= count as f32;
            average_position = average_position.normalize();
            average_position *= self.max_speed;
            let steer = Vec2::new(average_position.x, average_position.y) - self.velocity;
            self.apply_force(steer);
        }
    }

    pub fn cohersion (&mut self, boid_group: &BoidGroup) {
        let mut average_position = Vec3::new(0.0, 0.0, 0.0);
        let mut count = 0;
        for boid in boid_group.boids.iter() {
            let distance = self.distance_between(*boid);
            if boid != self && distance < self.local_radius {
                average_position += boid.position;
                count += 1;
            }
        }
        if count > 0 {
            average_position /= count as f32;
            let rotation = self.rotation_to_target(average_position.x, average_position.y);
            let steer = rotation - self.direction;
            self.apply_force(Vec2::new(steer, steer));
        }
    }

    pub fn rotation_to_target(&mut self, x: f32, y: f32) -> f32 {
        let dx = x - self.position.x;
        let dy = y - self.position.y;
        dy.atan2(dx).to_degrees()
    }

    pub fn rotate_towards(&mut self, x: f32, y: f32) {
        let dx = x - self.position.x;
        let dy = y - self.position.y;
        self.direction = dy.atan2(dx).to_degrees();
    }

    pub fn check_bounds(&mut self, width: f32, height: f32) {
        if self.position.x > width {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = width;
        }

        if self.position.y > height {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = height;
        }
    }

    fn distance_between(&mut self, boid: Boid) -> f32 {
        let dx = self.position.x - boid.position.x;
        let dy = self.position.y - boid.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance
    }

    fn nearest_boid (&mut self, boid_group: &BoidGroup) -> Boid {
        let mut nearest_boid = Boid::new(Vec3::new(0.0, 0.0, 0.0), 0.0, Vec2::new(0.0, 0.0));
        let mut nearest_distance = 100000.0;
        for boid in boid_group.boids.iter() {
            let distance = self.distance_between(*boid);
            if boid != self && distance < nearest_distance {
                nearest_distance = distance;
                nearest_boid = *boid;
            }
        }
        nearest_boid
    }


    

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }
}