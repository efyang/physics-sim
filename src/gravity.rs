use super::object::Object;
use super::vector::Vector;

const G_CONSTANT: f32 = 0.000_000_000_0674;

/// returns force on object 1, force on object 2
pub fn gravitational_forces(object1: &Object, object2: &Object) -> (Vector, Vector) {
    let distance_sq = object1.distance_to_squared(object2);
    let mass_product = object1.mass() * object2.mass();
    let magnitude = G_CONSTANT * (mass_product / distance_sq);
    let angle1 = object1.angle_to(object2);
    let angle2 = angle1 + ::std::f32::consts::PI;

    (Vector::new(magnitude, angle1), Vector::new(magnitude, angle2))
}
