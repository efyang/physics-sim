#![feature(test)]
#[cfg(test)]
extern crate test;
extern crate physics_sim;
use physics_sim::Vector;
use test::Bencher;

#[bench]
fn vector_addition(b: &mut Bencher) {
    let v1 = Vector::new(350., 23.2);
    let v2 = Vector::new(9.0007, 3.4781);
    b.iter(|| {
        v1 + v2
    });
}
