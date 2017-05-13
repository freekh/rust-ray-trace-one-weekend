mod vec3;
use vec3::{Vec3, dot};

mod ray;
use ray::Ray;

mod shape;
use shape::Sphere;

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

fn color(r: Ray) -> Vec3 {
  let t = 0.0; //hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
  if t > 0.0 {
    let n = (r.point_at_parameter(t) - 
             Vec3::new(0.0, 0.0, -1.0)).unit_vector();
    0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
  } else {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) +
      t*Vec3::new(0.5, 0.7, 1.0)
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
      let col = color(r);
      
      let ir: i32 = (col.r() * 255.99) as i32;
      let ig: i32 = (col.g() * 255.99) as i32;
      let ib: i32 = (col.b() * 255.99) as i32;
      println!("{r} {g} {b}", r = ir, g = ig, b = ib);
    }
  }
}
