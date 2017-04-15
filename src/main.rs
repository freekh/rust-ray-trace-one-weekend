
fn main() {
  let columns = 200;
  let rows = 100;

  println!("P3\n{columns} {rows}\n255\n",
           columns = columns,
           rows = rows);

  for row in 0..rows {
    for column in (0..columns).rev() {
      let r = (row as f64) / (rows as f64);
      let g = (column as f64) / (columns as f64);
      let b = 0.2;
      let ir: i32 = (r * 255.99) as i32;
      let ig: i32 = (g * 255.99) as i32;
      let ib: i32 = (b * 255.99) as i32;
      println!("{r} {g} {b}", r = ir, g = ig, b = ib);
    }
  }
}
