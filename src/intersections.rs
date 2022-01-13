use crate::ray::Ray;
use crate::shapes::Sphere;
use nalgebra::Vector3;

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
    scene: &'a [Sphere],
    ray: &'a Ray,
) -> Option<(&'a Sphere, f32)> {
    let mut distances = Vec::new();
    let mut nearest_object: Option<&Sphere> = None;
    let mut min_distance = f32::INFINITY;

    for obj in scene {
        let res = ray_sphere_intersection(ray, obj);
        match res {
            None => return None,
            Some(distance) => {
                println!("Ray intersected");
                distances.push(distance)
            }
        }
    }
    if distances.len() > 0 {
        println!("Distances vector is nonempty");
    } else {
        println!("Distances vector is empty");
    }

    for (pos, &distance) in distances.iter().enumerate() {
        println!("Ray has an intersection with a ball");
        if distance < min_distance {
            min_distance = distance;
            nearest_object = Some(&scene[pos]);
        }
    }
    match nearest_object {
        Some(obj) => return Some((obj, min_distance)),
        None => return None,
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use nalgebra::Vector3;

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
        let mut scene = Vec::new();

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

        let r = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        let res = nearest_intersected_object(&scene, &r);

        match res {
            Some((sphere, distance)) => {
                assert_eq!(sphere.radius, 1.0);
                assert_eq!(distance, 3.0);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_nearest_object_no_intersection() {
        let mut scene = Vec::new();

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

        let res = nearest_intersected_object(&scene, &r);

        match res {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
