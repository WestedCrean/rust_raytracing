use crate::ray::Ray;

pub trait Intersectable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32>;
}

#[derive(Default)]
struct Scene {
    hitable: Vec<Box<dyn Intersectable>>,
}
