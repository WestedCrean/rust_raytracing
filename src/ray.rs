use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }
    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}
