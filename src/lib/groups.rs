use crate::lib::Boid;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Clone)]
pub struct BoidGroup {
    pub boids: Vec<Boid>,
}

pub struct Floc {
    boids: Vec<Boid>,
    average_rotation: f32,
    average_position: Vec3,
    average_velocity: Vec2,
    floc_leader: Boid,
}

impl BoidGroup {
    
    pub fn new() -> Self {
        Self {
            boids: Vec::new(),
        }
    }

    pub fn add_boid(&mut self, boid: Boid) {
        self.boids.push(boid);
    }

}

impl Floc {

    pub fn new(floc_leader: Boid) -> Self {
        Self {
            boids: vec![floc_leader],
            average_rotation: floc_leader.direction,
            average_position: floc_leader.position,
            average_velocity: floc_leader.velocity,
            floc_leader,
        }
    }

    pub fn add_boid(&mut self, boid: Boid) {
        self.boids.push(boid);
    }

    pub fn remove_boid(&mut self, boid: Boid) {
        self.boids.retain(|b| *b != boid);
    }

    pub fn offset(&mut self) {
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(0..360);
        self.average_rotation = self.boids.iter().fold(0.0, |acc, boid| acc + boid.direction) / self.boids.len() as f32 + offset as f32;  
    }

    pub fn update(&mut self) {

        if self.boids.len() <= 1 {
            return;
        }

        self.average_rotation = self.boids.iter().fold(0.0, |acc, boid| acc + boid.direction) / self.boids.len() as f32;
        self.average_position = self.boids.iter().fold(Vec3::new(0.0, 0.0, 0.0), |acc, boid| acc + boid.position) / self.boids.len() as f32;
        self.average_velocity = self.boids.iter().fold(Vec2::new(0.0, 0.0), |acc, boid| acc + boid.velocity) / self.boids.len() as f32;
    }

}