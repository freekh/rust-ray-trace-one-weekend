mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

fn color(r: Ray) -> Vec3 {
  let unit_direction = r.direction().unit_vector();
  let t = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) +
    t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
  let columns = 200;
  let rows = 100;

  println!("P3\n{columns} {rows}\n255\n",
           columns = columns,
           rows = rows);

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


  for row in 0..rows {
    for column in (0..columns).rev() {
      let u = (column as f64) / (columns as f64);
      let v = (row as f64) / (rows as f64);

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
