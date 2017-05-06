mod vec3;
use vec3::Vec3;

mod ray;

fn main() {
  let columns = 200;
  let rows = 100;

  println!("P3\n{columns} {rows}\n255\n",
           columns = columns,
           rows = rows);

  for row in 0..rows {
    for column in (0..columns).rev() {
      let v = Vec3::new(
        (row as f64) / (rows as f64),
        (column as f64) / (columns as f64),
        0.2
      );

      let ir: i32 = (v.r() * 255.99) as i32;
      let ig: i32 = (v.g() * 255.99) as i32;
      let ib: i32 = (v.b() * 255.99) as i32;
      println!("{r} {g} {b}", r = ir, g = ig, b = ib);
    }
  }
}
