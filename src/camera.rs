use crate::math::Vec3;
use crate::general::Ray;

#[derive(Debug)]
pub struct Camera{
	pub pos: Vec3,
	// should be 3x3 matrix
	pub right: Vec3,
	pub up: Vec3,
	pub forward: Vec3,

	pub near: f64,
	pub far: f64,
	pub fov_deg: f64,
	pub aspect_ratio: f64,
	
	// plane points
	topLeft: Vec3,
	topRight: Vec3,
	botLeft: Vec3,
	botRight: Vec3,

	// width and heigh of plane
	horizontal: Vec3,
	vertical: Vec3,

	// TODO: update constructors to check input data
	// setters to update plane points
}

// let camera = camera::new()
// camera.ray()

impl Camera {
	pub fn new(pos: Vec3, right: Vec3, up: Vec3, forward: Vec3,
			   fov_deg: f64, aspect_ratio: f64, near: f64, far: f64) -> Camera {
		let d: f64 = 1.0;
		// center point of plane
		let center: Vec3 = pos + forward * d;		
		// plane sizes
		let height: f64 = 2.0 * (fov_deg.to_radians()/2.0).tan() * d;
		let width: f64 = height * aspect_ratio;

		let _topLeft: Vec3  = center + up * (height*0.5) - right * (width*0.5);
		let _topRight: Vec3 = center + up * (height*0.5) + right * (width*0.5);
		let _botLeft: Vec3  = center - up * (height*0.5) - right * (width*0.5);
		let _botRight: Vec3 = center - up * (height*0.5) + right * (width*0.5);

		let _horizontal: Vec3 = _botRight - _botLeft;
		let _vertical: Vec3 = _topLeft - _botLeft;

		Camera{
			pos: pos, right: right, up: up, forward: forward,
			fov_deg: fov_deg, aspect_ratio: aspect_ratio, near: near, far: far,
			topLeft: _topLeft, topRight: _topRight,
			botLeft: _botLeft, botRight: _botRight, 
			horizontal: _horizontal, vertical: _vertical,		
		}			
	}
	
	pub fn get_ray(&self, u: f64, v: f64) -> Ray {		
		Ray::new(self.pos, self.botLeft + u*self.horizontal + v*self.vertical)
	}

	pub fn default() -> Camera {
		Camera::new(
			Vec3::zero(), Vec3::new(1.0,0.0,0.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,0.0,-1.0),
			90.0, 1280.0/640.0, 0.001, 1000.0,
			)
	}	
}

/*
#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_add(){
		let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
		let result: Vec3 = v + Vec3::new(0.1,0.2,0.3);
		assert!(result.x == 1.1);
		assert!(result.y == 2.2);
		assert!(result.z == 3.3);
	}	

	#[test]
	fn test_sub(){
		let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
		let result: Vec3 = v - Vec3::new(0.1,0.2,0.3);
		assert!(result.x == 0.9);
		assert!(result.y == 1.8);
		assert!(result.z == 2.7);
	}

	#[test]
	fn test_mul(){
		let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
		let s: f64 = 3.0;
		let left: Vec3 = v * s;
		let right: Vec3 = s * v;
		// check side equality
		assert!(left.x == right.x);
		assert!(left.y == right.y);
		assert!(left.z == right.z);

		assert!(left.x == 3.0); 
		assert!(left.y == 6.0); 
		assert!(left.z == 9.0); 		
	}

	#[test]
	fn test_difficult(){
		let t: f64 = 0.5;
		let o: Vec3 = (1.0-t)*Vec3::new(1.0,1.0,1.0) + t*Vec3::new(0.5,0.7,1.0);
		assert!(o.x == 0.75);//0.5+0.5*0.5);
		assert!(o.y == 0.85);//0.5+0.5*0.7);
		assert!(o.z == 1.0);//0.5+0.5*1.0);
	}

	#[test]
	fn test_length(){
		let vector: Vec3 = Vec3::new(1.0, 4.0, 8.0);
		let length = vector.length();
		assert!(length == 9.0);	
	}

	#[test]
	fn test_normalize(){
		let vector: Vec3 = Vec3::new(3.0, 0.0, 4.0);
		let n: Vec3 = vector.normalized();
		assert!(n.x == 3.0/5.0);	
		assert!(n.y == 0.0);	
		assert!(n.z == 4.0/5.0);	
		assert!(n.length() == 1.0);	
	}		
	
}*/