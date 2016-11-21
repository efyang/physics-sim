use super::object::Object;
use super::gravity::gravitational_forces;

pub struct Universe {
    objects: Vec<Object>,
}

impl Universe {
    /// get the slice of all objects
    pub fn objects(&self) -> &[Object] {
        self.objects.as_slice()
    }

    /// add object to univers
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    /// returns list of indices removed
    pub fn update_state_repeat(&mut self, time: f64, iterations: usize) -> Vec<usize> {
        let mut removed_indices = Vec::new();
        let time_per_iter = time / iterations as f64;
        for _ in 0..iterations {
            removed_indices.extend(self.update_state(time_per_iter));
        }
        removed_indices
    }

    /// returns list of indices removed
    pub fn update_state(&mut self, time: f64) -> Vec<usize> {
        let mut removed_indices = Vec::new();
        // check for and handle any collisions
        for i in 0..self.objects.len() - 1 {
            for j in (i + 1..self.objects.len()).rev() {
                if self.objects[i].is_colliding(&self.objects[j]) {
                    if self.objects[i].more_massive(&self.objects[j]) {
                        let other = self.objects[j].clone();
                        self.objects[i].absorb_other(&other);
                        self.objects.remove(j);
                        removed_indices.push(j);
                    } else {
                        let other = self.objects[i].clone();
                        self.objects[j].absorb_other(&other);
                        self.objects.remove(i);
                        removed_indices.push(i);
                    }
                }
            }
        }
        // get all the forces and apply them
        // run in O(n^2) time
        for i in 0..self.objects.len() {
            for j in (0..self.objects.len()).filter(|&j| j != i) {
                let (f1, f2) = gravitational_forces(&self.objects[i], &self.objects[j]);
                self.objects[i].add_force(f1);
                self.objects[j].add_force(f2);
            }
            self.objects[i].update_state(time);
        }
        removed_indices
    }
}

impl Default for Universe {
    fn default() -> Universe {
        Universe { objects: Vec::new() }
    }
}
