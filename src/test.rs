mod math;
use math::Vec3;

// rustc [this_file] works fine..
// cargo not so much. why can't do "cargo run test.rs" ?
fn main() {
	//let v: Vec3 = Vec3{x: 1.0,y: 2.0,z:3.0};
	let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
	let length = v.length();
	println!("vector is {:?}", v);
	
	println!("length of vector is {}", length);
	
	v = v * 5.0;
	println!("vector is {:?}", v);
	
	v = v / 3.0;
	println!("vector is {:?}", v);
	
	v.normalize();
	println!("vector is {:?}", v);
	let length = v.length();
	println!("length of vector is {}", length);

	//let num = 2.0**2;
    //println!("number is {}", num);
}