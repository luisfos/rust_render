use crate::math::Vec3;

#[derive(Debug)]
// not sure if copy/clone is good idea here, used it to allow point along to clone itself
// copy means rust will implicitly copy on certain types (that go on stack not heap)
// clone means you can EXPLICITLY copy by calling .clone()
// we should maybe set to clone and not copy so we explicitly do them, for easier understanding and learning
#[derive(Copy,Clone)]
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