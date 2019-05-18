use std::cell::Cell;

fn main (){
  let a = Cell::new(1);
  a.set(2);
  let b = a;
  b.set(3);
  let c = b.get();
  let d = b.get();
  println!("{}", b.get());
  println!("{}", c);
  println!("{}", d);
}