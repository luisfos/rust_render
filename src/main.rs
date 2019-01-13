use std::error::Error;
use std::path::Path;
use std::fs::File;

// for creating image relative to exe path
use std::env;

// for writing to file (why is std::fs::file not enough?)
use std::io::prelude::*;


mod math;
mod general;
// why do we need to use 'rust_render', why not 'crate'?
// specifying crate name specifically (rust_render) will use the definition of `lib.rs`, which is only meant for external use (examples)
// instead we import the modules using 'mod' and then dictate the namespaces using 'use'
use crate::math::Vec3;
use crate::general::Ray;

// TODO divide image generation functions out of main
// TODO support other file formats
fn color(r: Ray) -> Vec3 {
    let unit_dir: Vec3 = r.direction.normalized();
    let t: f64 = 0.5*(unit_dir.y + 1.0);    
    (1.0-t)*Vec3::new(1.0,1.0,1.0) + t*Vec3::new(0.5,0.7,1.0)
}

fn main() {

	// TODO allow user to specify output image from cli, with default
    // unwrap retrieves the result of function (as usually it is packaged with an error)
    let p = env::current_dir().unwrap();
    println!("current dir is {}", p.display());
    //let path = Path::new("image.ppm");
    let path = p.join("image.ppm");//p.parent().unwrap();
    let display = path.display();

    let mut file = match File::create(&path){ // error handling
    	Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
    	Ok(file) => file,
    };

    let mut image = String::new();
    let nx = 200;
    let ny = 100;

    // initial data for image format ppm
    image.push_str(&format!("P3\n{} {}\n255\n", nx, ny));
    
    // image boundaries
    let bl_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::zero();
    
    // iterators are -1 to final. e.g 0..10 = [0,1,...,8,9]
    for j in (0..ny).rev() {
    	for i in 0..nx {
            // let col: Vec3 = Vec3::new(
            //     i as f64 / nx as f64,
            //     j as f64 / ny as f64,
            //     0.2
            // );
            let u: f64 = i as f64 / nx as f64;
            let v: f64 = j as f64 / ny as f64;
            let r: Ray = Ray::new(origin, bl_corner + u*horizontal + v*vertical);

            let col: Vec3 = color(r);

    		// let r: f64 = i as f64 / nx as f64;
    		// let g: f64 = j as f64 / ny as f64;    		
    		// let b: f64 = 0.2;
    		let ir = (255.99*col.x).round();
    		let ig = (255.99*col.y).round();
    		let ib = (255.99*col.z).round();
    		image.push_str(&format!("{} {} {}\n", ir,ig,ib));
    	}
    }
    

    match file.write_all(image.as_bytes()) {
    	Err(why) => panic!("couldn't write to {}: {}", display,	why.description()),
    	Ok(_) => println!("successfully wrote to {}", display),
    	
    }
}
