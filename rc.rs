use std::rc::Rc;

fn main (){
  let a = Rc::new(1);
  let b = a;
  //let c = a;
  println!("{}", b);
  println!("{}", Rc::strong_count(&b));
}