use crate::intersections::{Intersectable, IntersectionRecord};
use crate::ray::Ray;
use nalgebra::Vector3;
use sdl2::pixels::Color;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub color: Vector3<f32>,
    pub specular: f32,
    pub reflective: f32,
    pub refractive: f32,
    //ambient: Vector3<f32>,
    //diffuse: Vector3<f32>,
    //specular: Vector3<f32>
}

impl Sphere {
    pub fn new(
        center: Vector3<f32>,
        radius: f32,
        color: Vector3<f32>,
        specular: f32,
        reflective: f32,
        refractive: f32,
    ) -> Self {
        Sphere {
            center,
            radius,
            color,
            specular,
            reflective,
            refractive,
        }
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn specular(&self) -> f32 {
        self.specular
    }

    pub fn reflective(&self) -> f32 {
        self.reflective
    }

    pub fn refractive(&self) -> f32 {
        self.refractive
    }

    pub fn color_vector(&self) -> Vector3<f32> {
        self.color
    }

    pub fn get_color(&self) -> Color {
        let r = self.color[0];
        let g = self.color[1];
        let b = self.color[2];
        Color::RGB(r as u8, g as u8, b as u8)
    }
}

impl Intersectable for Sphere {
    fn center(&self) -> Vector3<f32> {
        self.center
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord> {
        // a = L - E ( Direction vector of ray, from start to end )
        let ray_to_sphere: Vector3<f32> = ray.origin() - self.center; // f = E - C ( Vector from center sphere to ray start )

        let a = ray.direction().dot(&ray.direction());
        let b = ray_to_sphere.dot(&ray.direction());
        let c: f32 = ray_to_sphere.dot(&ray_to_sphere) - self.radius.powi(2);

        // let delta = (b * b) - 4.0 * a * c;
        let delta = b.powi(2) - a * c;
        // println!("{}x^2 + {}x + {}", a, b, c);
        // println!("delta: {}", delta);

        if delta > 0.0 {
            let delta_squared = f32::sqrt(delta);
            let t = (-b + -delta_squared) / a;

            if t_min < t && t < t_max {
                let p = ray.point_at_parameter(t);
                return Some(IntersectionRecord {
                    intersection_point: t,
                    intersection_vector: p,
                    object_center: self.center(),
                    object_color: self.get_color(),
                    object_specular: self.specular(),
                    object_reflective: self.reflective(),
                    object_refractive: self.refractive(),
                });
            }

            let t = (-b + delta_squared) / a;

            if t_min < t && t < t_max {
                let p = ray.point_at_parameter(t);
                return Some(IntersectionRecord {
                    intersection_point: t,
                    intersection_vector: p,
                    object_center: self.center(),
                    object_color: self.get_color(),
                    object_specular: self.specular(),
                    object_reflective: self.reflective(),
                    object_refractive: self.refractive(),
                });
            }
        }
        None
    }
}
