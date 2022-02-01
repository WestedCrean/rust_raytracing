use nalgebra::Vector3;

use crate::intersections::Intersectable;
use crate::lights::Light;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn push(&mut self, object: impl Intersectable + 'static) {
        self.objects.push(Box::new(object))
    }

    pub fn add_light(&mut self, light: impl Light + 'static) {
        self.lights.push(Box::new(light))
    }

    pub fn get_nth_element_center(&self, n: i32) -> Option<Vector3<f32>> {
        if let Some(obj) = self.objects.get(n as usize) {
            return Some(obj.center());
        }

        None
    }
}
