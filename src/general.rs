use crate::math::Vec3;

pub struct Scene{
	
}


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

pub struct HitRecord {
	pub t: f64,
	pub p: Vec3,
	pub normal: Vec3,
}

trait Hitable {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

//#[derive(Copy,Clone)]
pub struct Sphere{
	pub pos: Vec3,
	pub rad: f64,
}

impl Hitable for Sphere{
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let oc: Vec3 = r.origin - self.pos;//*center;
		let a: f64 = r.direction.dot(&r.direction);
		let b: f64 = 2.0 * oc.dot(&r.direction);
		let c: f64 = oc.dot(&oc) - self.rad*self.rad;
		let discriminant: f64 = b*b - 4.0*a*c;
		// no real collisions
		if discriminant > 0.0 {
			let solve: f64 = (-b - discriminant.sqrt()) / (2.0*a);
			if solve < t_max && solve > t_min {
				rec.t = solve;
				rec.p = r.point_along(rec.t);
				rec.normal = (rec.p - self.pos) / self.rad;
				return true;
			}
			let solve: f64 = (-b + discriminant.sqrt()) / (2.0*a);
			if solve < t_max && solve > t_min {
				rec.t = solve;
				rec.p = r.point_along(rec.t);
				rec.normal = (rec.p - self.pos) / self.rad;
				return true;
			}
		}
		false
	}
}

pub struct Plane{
	// 3 points 
	// normal and distance (infinite)	
	// normal, distance + width & height (finite)
	pub normal: Vec3,
	pub d: f64,
}