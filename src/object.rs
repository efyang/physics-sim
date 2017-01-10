use super::vector::Vector;
use super::point::Point;

#[derive(Clone, Debug)]
pub struct Object {
    mass: f64,
    radius: f64,
    density: f64,
    velocity: Vector,
    position: Point,
    acting_forces: Vector,
}

impl Object {
    pub fn new(mass: f64, density: f64, velocity: Vector, position: Point) -> Self {
        Object {
            mass: mass,
            radius: mass_to_radius(mass, density),
            density: density,
            velocity: velocity,
            position: position,
            acting_forces: Vector::default(),
        }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    pub fn add_force(&mut self, force: Vector) {
        self.acting_forces = self.acting_forces + force;
    }

    pub fn update_state(&mut self, time: f64) {
        let force_sum = self.acting_forces;
        let acceleration = force_sum / self.mass;
        self.update_position(acceleration, time);
        self.update_velocity(acceleration, time);
        self.acting_forces = Vector::default();
    }

    fn update_position(&mut self, acceleration: Vector, time: f64) {
        self.position = self.position + self.velocity * time + acceleration * time.powi(2) / 2.;
    }

    fn update_velocity(&mut self, acceleration: Vector, time: f64) {
        self.velocity = self.velocity + acceleration * time;
    }

    pub fn force_sum(&self) -> Vector {
        self.acting_forces
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn distance_to_squared(&self, other: &Object) -> f64 {
        self.position.distance_to_squared(&other.position)
    }

    pub fn distance_to(&self, other: &Object) -> f64 {
        self.distance_to_squared(other).sqrt()
    }

    pub fn angle_to(&self, other: &Object) -> f64 {
        let (x, y) = (other.position.x - self.position.x, other.position.y - self.position.y);
        y.atan2(x)
    }

    pub fn increment_velocity(&mut self, increment: Vector) {
        self.velocity = self.velocity + increment;
    }

    pub fn is_colliding(&self, other: &Object) -> bool {
        let collision_distance = self.radius + other.radius;
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
        self.mass += other.mass; // make this change density too at some point
        self.radius = mass_to_radius(self.mass, self.density);
    }
}

fn mass_to_radius(mass: f64, density: f64) -> f64 {
    (((mass / density) * 0.75) / ::std::f64::consts::PI).powf(1. / 3.)
}
