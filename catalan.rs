// 右、左を持つ構造体を扱う
// まず足し算だけを使う想定でコードを書きます
// まず演算子を使う
// 演算子を使い切るまで、演算子の使った数 >= 数字を使った数になる

enum Op {
    Plus,
    Minus,
    Mul,
    Div
}

enum Tree<T> {
    Leaf(T),
    Node(Op, Box<Tree<T>>, Box<Tree<T>>)
}

fn is_finish(op_cnt: i32, n_cnt: i32) -> bool {
  op_cnt == 3 && n_cnt == 3
}

fn catalan_tree(tree: Tree<i32>, op_cnt: i32, n_cnt: i32){
  //println!("{} {}", op_cnt, n_cnt);
  if is_finish(op_cnt, n_cnt){
    println!("{} {}", op_cnt, n_cnt)
    // TODO: Treeを完成させる
    // TODO: 10になるか検算する
    // TODO: 10になったらtreeの中身を印字
  }
  // 演算子を積む
  if op_cnt < 3 {
    catalan_tree(
      Tree::Node(Op::Plus, Box::new(Tree::Leaf(4)), Box::new(Tree::Leaf(4))),
      op_cnt + 1, n_cnt)
  }
  // 数字を積む
  if op_cnt > n_cnt {
    catalan_tree(
      Tree::Node(Op::Plus, Box::new(Tree::Leaf(4)), Box::new(Tree::Leaf(4))),
      op_cnt, n_cnt + 1)
  }
}

fn main(){
  catalan_tree(
    Tree::Node(Op::Plus, Box::new(Tree::Leaf(4)), Box::new(Tree::Leaf(4))),
    0,
    0
  )
}
