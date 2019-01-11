/* TODO
setup external file for testing - examples? multiple targets? why is this so complex
see if &self can be named anything else - is it like python classes?
see if "&mut self" is equivalent of mutable &self and correct syntax

*/

use std::ops::{Mul,Div};

// allows println to show struct - https://doc.rust-lang.org/book/ch05-02-example-structs.html
#[derive(Debug)]
pub struct Vec3{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: f64) -> Vec3 {
		Vec3 { 
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
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
	
	//we need to read and write - mutable reference borrow
	pub fn normalize(&mut self) -> () {		
		let length = self.length();
		self.x /= length;
		self.y /= length;
		self.z /= length;		
	}
}

// impl ops::Add for Vec3 {
// 	type Output
// }
