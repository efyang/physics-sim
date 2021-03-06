#![feature(test)]
#[cfg(test)]
extern crate test;
extern crate physics_sim;
use physics_sim::*;
use test::Bencher;

const OBJECT_MASS: f64 = 291083.10129;
const UNIVERSE_OBJECTS: usize = 10;
const SPACING: f64 = 1000000.;

#[bench]
fn universe_creation(b: &mut Bencher) {
    b.iter(|| {
        let mut universe = Universe::default();
        for i in 0..UNIVERSE_OBJECTS {
            universe.add_object(Object::new(OBJECT_MASS, Vector::default(), Point::new(i as f64 * SPACING, i as f64 * SPACING)));
        }
    });
}

#[bench]
fn universe_update(b: &mut Bencher) {
    b.iter(|| {
        let mut universe = Universe::default();
        for i in 0..UNIVERSE_OBJECTS {
            universe.add_object(Object::new(OBJECT_MASS, Vector::default(), Point::new(i as f64 * SPACING, i as f64 * SPACING)));
        }
        universe.update_state(1.);
    });
}
