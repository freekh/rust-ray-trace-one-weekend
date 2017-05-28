extern crate rand;
use rand::{random, Closed01};

use std::env;
use std::f64;
use std::time::Instant;
use std::f64::consts;

// 

mod vec3;
mod ray;
mod camera;
mod shape;
mod material;

use vec3::Vec3;
use ray::Ray;
use camera::Camera;
use shape::{Sphere, Shape, Shapes};
use material::*;

//
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

fn color<S>(r: Ray, world: &Shapes<S>, depth: i32) -> Vec3 where S: Shape + Sized{
  let maybe_hit = world.hit(r, 0.001, f64::MAX);
  match maybe_hit {
    Some(hit) => {
      if depth >= 50 { // TODO: not ideal, but could not see how to use && with if let
        Vec3::new(0.0, 0.0, 0.0) 
      } else if let Some((attunation, scattered)) = hit.material.scatter(r, &hit) {
        attunation * color(scattered, world, depth + 1)
      } else {
        Vec3::new(0.0, 0.0, 0.0) 
      }
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
  let start = Instant::now();

  let nx = 400;
  let ny = 200;
  let ns = env::args().nth(1).map(|a| a.parse::<i32>().unwrap()).unwrap_or(100);
  
  debug!("Piping ppm to stdout...\n");

  println!("P3\n{nx} {ny}\n255\n",
           nx = nx,
           ny = ny);

  let lookfrom = Vec3::new(3.0, 3.0, 2.0);
  let lookat = Vec3::new(0.0, 0.0, -1.0);
  
  let dist_to_focus = (lookfrom - lookat).length();

  let camera = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    20.0,
    nx as f64 / ny as f64,
    1.0,
    dist_to_focus
  );

  let m1 = &Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
  let m2 = &Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
  let m3 = &Metal::new(Vec3::new(0.8, 0.6, 0.2));
  let m4 = &Dielectric::new(1.5);

  let world = Shapes(vec!(
    Sphere::new(
      Vec3::new(0.0, 0.0, -1.0), 
      0.5,
      m1
    ),
    Sphere::new(
      Vec3::new(0.0, -100.5, -1.0), 
      100.0,
      m2
    ),
    Sphere::new(
      Vec3::new(1.0, 0.0, -1.0), 
      0.5,
      m3
    ),
    Sphere::new(
      Vec3::new(-1.0, 0.0, -1.0), 
      0.5,
      m4
    ),
    Sphere::new(
      Vec3::new(-1.0, 0.0, -1.0), 
      -0.45,
      m4
    )
  ));


  for y in (0..ny).rev() {
    for x in 0..nx {
      let col = 
        (0..ns).fold(Vec3::new(0.0, 0.0, 0.0), |col, _| {
          let Closed01(u_rand) = random::<Closed01<f64>>();
          let Closed01(v_rand) = random::<Closed01<f64>>();
          let u = (x as f64 + u_rand) / (nx as f64);
          let v = (y as f64 + v_rand) / (ny as f64);
          let r = camera.get_ray(u, v);
          col + color(r, &world, 0)
        }) / (ns as f64);
      let gamma_corrected = Vec3::new(
        col.r().sqrt(),
        col.g().sqrt(),
        col.b().sqrt()
      );
      
      let ir: i32 = (gamma_corrected.r() * 255.99) as i32;
      let ig: i32 = (gamma_corrected.g() * 255.99) as i32;
      let ib: i32 = (gamma_corrected.b() * 255.99) as i32;

      debug!("\r\r\r\r{percent}%", percent = (x + nx * (ny - y - 1)) * 100 / (nx * ny - 1));

      println!("{r} {g} {b}", r = ir, g = ig, b = ib);
    }
  }

  let elapsed = start.elapsed();
  debug!(
    "\r\r\r\rDone in {seconds}.{fraction}!\n",
    seconds = elapsed.as_secs(),
    fraction = elapsed.subsec_nanos()
  )
}
