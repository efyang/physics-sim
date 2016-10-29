use std::time::Duration;
use super::vector::Vector;
use super::point::Point;

#[derive(Clone, Debug)]
pub struct Object {
    mass: f64,
    velocity: Vector,
    position: Point,
    acting_forces: Vec<Vector>,
    force_sum_cache: Option<Vector>,
}

impl Object {
    pub fn new(mass: f64, velocity: Vector, position: Point) -> Self {
        Object {
            mass: mass,
            velocity: velocity,
            position: position,
            acting_forces: Vec::new(),
            force_sum_cache: None,
        }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    pub fn add_force(&mut self, force: Vector) {
        self.acting_forces.push(force);
    }

    pub fn update_state(&mut self, time: Duration) {
        let force_sum = self.force_sum_caching();
        let acceleration = force_sum / self.mass;
        let time_s = time.as_secs() as f64;
        self.update_position(acceleration, time_s);
        self.update_velocity(acceleration, time_s);
        self.acting_forces.clear();
        self.force_sum_cache = None;
    }

    fn update_position(&mut self, acceleration: Vector, time: f64) {
        self.position = self.position + self.velocity * time + acceleration * time.powi(2) / 2.;
    }

    fn update_velocity(&mut self, acceleration: Vector, time: f64) {
        self.velocity = self.velocity + acceleration * time;
    }

    pub fn force_sum_caching(&mut self) -> Vector {
        if self.force_sum_cache.is_some() {
            self.force_sum_cache.unwrap()
        } else {
            let sum = self.acting_forces.iter().cloned().sum();
            self.force_sum_cache = Some(sum);
            sum
        }
    }

    pub fn force_sum(&self) -> Vector {
        self.force_sum_cache.unwrap_or(self.acting_forces.iter().cloned().sum())
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn get_radius(&self) -> f64 {
        (self.mass / ::std::f64::consts::PI).sqrt()
    }

    pub fn distance_to(&self, other: &Object) -> f64 {
        self.position.distance_to(&other.position)
    }

    pub fn angle_to(&self, other: &Object) -> f64 {
        let (x, y) = (other.position.x - self.position.x, other.position.y - self.position.y);
        y.atan2(x)
    }

    pub fn increment_velocity(&mut self, increment: Vector) {
        self.velocity = self.velocity + increment;
    }

    pub fn is_colliding(&self, other: &Object) -> bool {
        let collision_distance = self.get_radius() + other.get_radius();
        let distance = self.distance_to(other);
        distance < collision_distance
    }

    pub fn more_massive(&self, other: &Object) -> bool {
        self.mass > other.mass
    }

    // if colliding eliminate the smaller one and add its mass to larger, and change velocity by
    // equivalent of time it would've taken for center of masses to hit each other or get close
    // enough to basically be comsidered as such
    /// assumes that the other is the smaller object
    pub fn absorb_other(&mut self, other: &Object) {
        let force_add = other.force_sum();
        // combined distance / combined magnitudes of velocities - very much approximated
        // TODO: make more accurate (factor in acceleration and angle)
        let time_in_collision = self.distance_to(other) /
                                (self.velocity.magnitude() + other.velocity.magnitude());
        self.position = self.position + self.velocity * time_in_collision;
        self.velocity = self.velocity + (force_add / self.mass) * time_in_collision;
        self.mass += other.mass;
    }
}
