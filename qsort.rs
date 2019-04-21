
fn qsort(ns: Vec<i32>) -> Vec<i32> {
  if ns.len() < (1 as usize) {
    return ns;
  }

  let pivot = ns[0];
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();
  
  for i in ns {
    if pivot == i {
      continue;
    } else if pivot > i {
      left.push(i)
    } else {
      right.push(i)
    }
  }
  let mut r = qsort(left.clone());
  r.append(&mut vec![pivot]);
  r.append(&mut qsort(right.clone()));

  return r;
}

fn main(){
  let ns: Vec<i32> = vec![5,6,3,8,1,7,10,2,4,9];
  let r = qsort(ns.clone());
  for i in r {
    print!("{} ", i);
  }
}
