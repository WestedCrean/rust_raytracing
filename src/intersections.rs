use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::Sphere;
use sdl2::pixels::Color;

use nalgebra::Vector3;
pub struct IntersectionRecord {
    pub intersection_point: f32,
    pub intersection_vector: Vector3<f32>,
    pub object_center: Vector3<f32>,
    pub object_color: Color,
}
pub trait Intersectable: Sync {
    fn center(&self) -> Vector3<f32>;
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord>;
}

pub fn ray_sphere_intersection(ray: &Ray, sphere: &Sphere) -> Option<f32> {
    let radius = sphere.radius;

    let ray_direction: Vector3<f32> = ray.direction(); // d = L - E ( Direction vector of ray, from start to end )
    let ray_to_sphere: Vector3<f32> = ray.origin() - sphere.center; // f = E - C ( Vector from center sphere to ray start )

    let a = ray_direction.dot(&ray_direction);
    let b = 2.0 * ray_to_sphere.dot(&ray_direction);
    let c: f32 = ray_to_sphere.dot(&ray_to_sphere) - (radius * radius);
    let delta = (b * b) - 4.0 * a * c;

    // println!("{}x^2 + {}x + {}", a, b, c);
    // println!("delta: {}", delta);

    if delta > 0.0 {
        let t1 = (-b + f32::sqrt(delta)) / (2.0 * a);
        let t2 = (-b - f32::sqrt(delta)) / (2.0 * a);

        if (t1 > 0.0) & (t2 > 0.0) {
            return Some(f32::min(t1, t2));
        }
    } else if delta == 0.0 {
        return Some(-b / 2.0);
    }
    return None;
}

pub fn nearest_intersected_object<'a>(
    scene: &Scene,
    ray: &'a Ray,
    min_distance: f32,
    max_distance: f32,
) -> Option<IntersectionRecord> {
    let mut nearest_object_distance = max_distance;
    let mut intersect_anything: Option<IntersectionRecord> = None;

    for obj in scene.objects.iter() {
        if let Some(intersection) = obj.intersect(ray, min_distance, nearest_object_distance) {
            nearest_object_distance = intersection.intersection_point;
            intersect_anything = Some(intersection);
        }
    }

    intersect_anything
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, scene::Scene};
    use nalgebra::Vector3;
    use sdl2::pixels::Color;

    use crate::intersections::{nearest_intersected_object, ray_sphere_intersection, Sphere};

    #[test]
    fn test_ray_sphere_two_intersections() {
        let r1 = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        let sphere = Sphere {
            center: Vector3::new(4.0, 0.0, 0.0),
            radius: 1.0,
            color: Vector3::new(0.0, 0.0, 0.0),
        };

        let res = ray_sphere_intersection(&r1, &sphere);

        match res {
            Some(val) => assert_eq!(val, 3.0 as f32),
            None => assert!(false),
        }
    }

    #[test]
    fn ray_sphere_intersection_intersection() {
        let r2 = Ray::new(Vector3::new(0.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 0.0));

        let sphere = Sphere {
            center: Vector3::new(4.0, 0.0, 0.0),
            radius: 1.0,
            color: Vector3::new(0.0, 0.0, 0.0),
        };

        let res = ray_sphere_intersection(&r2, &sphere);

        match res {
            Some(val) => assert_eq!(val, 4.0 as f32),
            None => assert!(false),
        }
    }

    #[test]
    fn ray_sphere_intersection_no_intersection() {
        let r3 = Ray::new(Vector3::new(0.0, 0.0, 2.0), Vector3::new(1.0, 0.0, 0.0));

        let sphere = Sphere {
            center: Vector3::new(4.0, 0.0, 0.0),
            radius: 1.0,
            color: Vector3::new(0.0, 0.0, 0.0),
        };

        let res = ray_sphere_intersection(&r3, &sphere);

        match res {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }

    #[test]
    fn test_nearest_object_trivial() {
        let mut scene = Scene::default();

        scene.push(Sphere {
            center: Vector3::new(7.0, 0.0, 0.0),
            radius: 1.2,
            color: Vector3::new(255.0, 255.0, 255.0),
        });

        scene.push(Sphere {
            center: Vector3::new(4.0, 0.0, 0.0),
            radius: 1.0,
            color: Vector3::new(0.0, 0.0, 0.0),
        });

        let r = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        let res = nearest_intersected_object(&scene, &r, 0.01, 10.0);

        match res {
            Some(intersection) => {
                assert_eq!(
                    intersection.object_color,
                    Color::RGB(0.0 as u8, 0.0 as u8, 0.0 as u8)
                );
                assert_eq!(intersection.intersection_point, 3.0);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_nearest_object_no_intersection() {
        let mut scene = Scene::default();

        scene.push(Sphere {
            center: Vector3::new(7.0, 0.0, 0.0),
            radius: 1.2,
            color: Vector3::new(0.0, 0.0, 0.0),
        });

        scene.push(Sphere {
            center: Vector3::new(4.0, 0.0, 0.0),
            radius: 1.0,
            color: Vector3::new(0.0, 0.0, 0.0),
        });

        let r = Ray::new(Vector3::new(0.0, 0.0, 3.0), Vector3::new(1.0, 0.0, 0.0));

        let res = nearest_intersected_object(&scene, &r, 0.01, 10.0);

        match res {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
