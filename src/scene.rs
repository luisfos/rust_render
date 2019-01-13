use crate::math::Vec3;

#[derive(Debug)]
pub struct Ray{
	pub origin: Vec3,
	pub direction: Vec3,	
}

impl Ray {
	pub fn new(_origin: Vec3, _direction: Vec3) -> Ray {
		Ray {
			origin: _origin,
			direction: _direction,
		}
	}
	pub fn point_along(self, _t: f64) -> Vec3 {
		self.origin + self.direction*_t
	}
}