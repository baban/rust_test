// 右、左を持つ構造体を扱う
// まず足し算だけを使う想定でコードを書きます
// まず演算子を使う
// 演算子を使い切るまで、演算子の使った数 >= 数字を使った数になる

use Tree::Node;

enum Op {
    Plus,
    Minus,
    Mul,
    Div
}

enum Tree<T> {
    Leaf(T),
    Node(Op, Option<Box<Tree<T>>>, Option<Box<Tree<T>>>),
}

fn is_finish(op_cnt: i32, n_cnt: i32) -> bool {
  op_cnt == 3 && n_cnt == 3
}

fn catalan_tree(mut path: Vec<i32>, op_cnt: i32, n_cnt: i32){
  if is_finish(op_cnt, n_cnt){
    println!("{} {}", op_cnt, n_cnt);
    path.push(1);
    for d in path.clone() { print!("{} ", d) }
    println!("");
    // TODO: Treeを完成させる
    // TODO: 10になるか検算する
    // TODO: 10になったらtreeの中身を印字
    return
  }
  // 演算子を積む
  if op_cnt < 3 {
    let mut left = path.clone();
    left.push(2);
    catalan_tree(left, op_cnt + 1, n_cnt)
  }
  // 数字を積む
  if op_cnt > n_cnt {
    let mut up = path.clone();
    up.push(1);
    catalan_tree(up, op_cnt, n_cnt + 1)
  }
}

fn build_tree(path: Vec<i32>, mut tree: Tree<i32>){
  if path.len() == 0 {
    return
  }
  let head = path[0];
  // 演算子では最初の枝に、次の演算子を積む
  // 数値の場合は、左の枝、右の枝、親の右の枝の順で空いているところを探して
  /*match head {
    2 => Tree::Node(Op::Plus, None, None),
    1 => Tree::Node(Op::Plus, None, None),
    _ => Tree::Node(Op::Plus, None, None),
  };*/
}

fn main(){
  //catalan_tree(vec![].clone(), 0, 0);
  //let n = Tree::Node(Op::Plus, None, None);
  let n = Tree::Node(
    Op::Plus,
    Some(Box::new(Tree::Leaf(4))),
    Some(Box::new(Tree::Leaf(4))));
  let n2 = Tree::Node(
    Op::Plus,
    None,
    Some(Box::new(Tree::Leaf(4))));
  let n3 = Tree::Node(
    Op::Plus,
    Some(Box::new(Tree::Leaf(4))),
    None);
  let n3: Tree<i32> = Tree::Node(
    Op::Plus,
    None,
    None);

  //build_tree(vec![2,2,2,1,1,1,1])
}
