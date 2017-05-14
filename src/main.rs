extern crate rand;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod camera;
use camera::Camera;

mod shape;
use shape::{Sphere, Shape, Shapes};

use std::f64;

macro_rules! debug {
  ($($arg:tt)*) => (
    {
      use std::io::prelude::*;
      if let Err(e) = write!(&mut ::std::io::stderr(), "{}", format_args!($($arg)*)) {
        panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
      }
    }
  )
}

fn color<S>(r: Ray, world: &Shapes<S>) -> Vec3 where S: Shape {
  let maybe_hit = world.hit(r, 0.0, f64::MAX);
  match maybe_hit {
    Some(hit) => {
      0.5 * Vec3::new(
        hit.normal.x() + 1.0,
        hit.normal.y() + 1.0,
        hit.normal.z() + 1.0
      )
    }
    None => {
      let unit_direction = r.direction().unit_vector();
      let t = 0.5 * (unit_direction.y() + 1.0);
      (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) +
        t*Vec3::new(0.5, 0.7, 1.0)
    }
  }
}

//

fn main() {
  let nx = 200;
  let ny = 100;
  let ns = 100;
  
  debug!("Piping ppm to stdout...\n");

  println!("P3\n{nx} {ny}\n255\n",
           nx = nx,
           ny = ny);
  
  let world = Shapes(vec!(
    Sphere::new(
      Vec3::new(0.0, 0.0, -1.0), 
      0.5
    ),
    Sphere::new(
      Vec3::new(0.0, -100.5, -1.0), 
      100.0
    )
  ));

  let camera = Camera::new();

  for y in (0..ny).rev() {
    for x in 0..nx {

      let col = 
        (0..ns).fold(Vec3::new(0.0, 0.0, 0.0), |col, _| {
          let u = (x as f64 + rand::random::<f64>()) / (nx as f64);
          let v = (y as f64 + rand::random::<f64>()) / (ny as f64);
          let r = camera.get_ray(u, v);
          col + color(r, &world)
        }) / (ns as f64);
      
      let ir: i32 = (col.r() * 255.99) as i32;
      let ig: i32 = (col.g() * 255.99) as i32;
      let ib: i32 = (col.b() * 255.99) as i32;
      println!("{r} {g} {b}", r = ir, g = ig, b = ib);
    }
  }
  debug!("\r\r\r\rDone!\n")
}
