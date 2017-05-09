mod vec3;
use vec3::{Vec3, dot};

mod ray;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
  let oc = r.origin() - center;
  let a = dot(r.direction(), r.direction());
  let b = 2.0 * dot(oc, r.direction());
  let c = dot(oc, oc) - radius * radius;
  let discriminant = b * b - 4.0 * a * c; // "the square part"
  discriminant > 0.0
}

fn color(r: Ray) -> Vec3 {
  if (hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r)) {
    Vec3::new(1.0, 0.0, 0.0)
  } else {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) +
      t*Vec3::new(0.5, 0.7, 1.0)
  }
}

fn main() {
  let nx = 200;
  let ny = 100;

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
