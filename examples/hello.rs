//mod math;
//extern crate rust_render;
//mod ../src/math;

// want to import math::Vec3
//use rust_render;
use rust_render::math::Vec3;


fn main() {
	println!("I'm an example");	

	let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
	let length = v.length();
	println!("vector is {:?}", v);
	println!("length of vector is {}", length);	
	
	v = v * 5.0;
	println!("multiplied by 5, now: {:?}", v);
	
	v = v / 3.0;
	println!("divided by 3, now: {:?}", v);
	
	v.normalize();
	println!("normalized vector is {:?}", v);
	let length = v.length();
	println!("length of normalized vector is {}", length);
}