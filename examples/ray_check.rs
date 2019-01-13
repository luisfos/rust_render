use rust_render::general::Ray;


fn main() {
	println!("I'm an example");	
	let v1: Ray = Vec3::new(1.0, 2.0, 3.0);
	let v2: Ray = Vec3::new(1.0, 2.0, 3.0);
	let r: Ray = Ray:new(v1, v2);	
	
	println!("Ray is {:?}", r);	
}