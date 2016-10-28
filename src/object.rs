use std::time::Duration;
use super::vector::Vector;
use super::point::Point;

#[derive(Clone, Debug)]
pub struct Object {
    mass: f64,
    velocity: Vector,
    position: Point,
    acting_forces: Vec<Vector>,
}

impl Object {
    pub fn add_force(&mut self, force: Vector) {
        self.acting_forces.push(force);
    }

    pub fn update_state(&mut self, time: Duration) {
        let force_sum: Vector = self.acting_forces.iter().cloned().sum();
        let acceleration = force_sum / self.mass;
        let time_s = time.as_secs() as f64;
        self.update_position(acceleration, time_s);
        self.update_velocity(acceleration, time_s);
        self.acting_forces.clear();
    }

    fn update_position(&mut self, acceleration: Vector, time: f64) {
        self.position = self.position + self.velocity * time + acceleration * time.powi(2) / 2.;
    }

    fn update_velocity(&mut self, acceleration: Vector, time: f64) {
        self.velocity = self.velocity + acceleration * time;
    }

    pub fn get_radius(&self) -> f64 {
        (self.mass / ::std::f64::consts::PI).sqrt()
    }

    pub fn is_colliding(&self, other: Object) -> bool {
        let collision_distance = self.get_radius() + other.get_radius();
        let distance = self.position.distance_to(&other.position);
        distance < collision_distance
    }

    pub fn absorb_other(&self, other: Object) {
        unimplemented!()
    }

    // if colliding eliminate the smaller one and add its mass to larger, and change velocity by
    // equivalent of time it would've taken for center of masses to hit each other or get close
    // enough to basically be comsidered as such
}
