mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod shape;
use shape::{Sphere, Shape, Shapes};

use std::f64;


macro_rules! debug {
  ($($arg:tt)*) => (
    {
      use std::io::prelude::*;
      if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
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
  
  debug!("Piping ppm to stdout...");

  println!("P3\n{nx} {ny}\n255\n",
           nx = nx,
           ny = ny);

  let lower_left_corner = Vec3::new(
    -2.0, -1.0, -1.0
  );
  let horizontal = Vec3::new(
    4.0, 0.0, 0.0
  );
  let vertical = Vec3::new(
    0.0, 2.0, 0.0
  );
  let origin = Vec3::new(
    0.0, 0.0, 0.0
  );
  
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

  for y in (0..ny).rev() {
    for x in 0..nx {
      let u = (x as f64) / (nx as f64);
      let v = (y as f64) / (ny as f64);

      let r = Ray::new(
        origin, 
        lower_left_corner + 
          u * horizontal +
          v * vertical
      );
      let col = color(r, &world);
      
      let ir: i32 = (col.r() * 255.99) as i32;
      let ig: i32 = (col.g() * 255.99) as i32;
      let ib: i32 = (col.b() * 255.99) as i32;
      println!("{r} {g} {b}", r = ir, g = ig, b = ib);
    }
  }
}
