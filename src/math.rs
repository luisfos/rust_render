/* TODO
setup external file for testing - examples? multiple targets? why is this so complex
see if &self can be named anything else - is it like python classes?

*/

use std::ops::{Mul,Div,Add,Sub};

// allows println to show struct - https://doc.rust-lang.org/book/ch05-02-example-structs.html
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec3{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	// maybe should be references to prevent copying?:
	// fn mul(&self, other: &f64)
	fn mul(self, other: f64) -> Vec3 {
		Vec3 { 
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}
}
impl Mul<Vec3> for f64 {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 { 
			x: other.x * self,
			y: other.y * self,
			z: other.z * self,
		}
	}
}


impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, other: f64) -> Vec3 {
		Vec3 { 
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 { 
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 { 
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}


impl Vec3 {
	pub fn new(sx: f64, sy: f64, sz: f64) -> Vec3 {
		Vec3 {x: sx, y: sy, z: sz}
	}

	pub fn zero() -> Vec3 {
		Vec3 {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
	
	// borrows the vector to return another value
	pub fn length(&self) -> f64 {
		let val = self.x*self.x + self.y*self.y + self.z*self.z;
		val.sqrt()
	}

	pub fn length2(&self) -> f64 {
		self.x*self.x + self.y*self.y + self.z*self.z		
	}	
	
	pub fn dot(&self, other: &Vec3) -> f64 {
		self.x*other.x + self.y*other.y + self.z*other.z
	}
	
	pub fn normalized(self) -> Vec3 {
		//Vec3::new(self.x, self.y, self.z) / self.length()
		self / self.length()
	}
	//we need to read and write - mutable reference borrow
	// pub fn normalize(&mut self) -> () {		
	// 	let length = self.length();
	// 	self.x /= length;
	// 	self.y /= length;
	// 	self.z /= length;		
	// }
}


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
	
}