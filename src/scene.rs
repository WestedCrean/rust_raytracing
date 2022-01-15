use nalgebra::Vector3;

use crate::intersections::Intersectable;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn push(&mut self, object: impl Intersectable + 'static) {
        self.objects.push(Box::new(object))
    }

    pub fn get_nth_element_center(&self, n: i32) -> Option<Vector3<f32>> {
        if let Some(obj) = self.objects.get(n as usize) {
            return Some(obj.center());
        }

        None
    }
}
