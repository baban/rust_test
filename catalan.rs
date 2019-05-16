// 右、左を持つ構造体を扱う
// まず足し算だけを使う想定でコードを書きます
// まず演算子を使う
// 演算子を使い切るまで、演算子の使った数 >= 数字を使った数になる

struct TreeNode {
  left: Tree,
  right: Tree
}

enum Tree {
  Node(Box<TreeNode>),
  Leaf { value: i32 }
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

/*
fn build_tree(path: Vec<i32>, mut tree: Tree<i32>){
  if path.len() == 0 {
    return
  }
  let head = path[0];
  // 演算子(2)では最初の枝に、次の演算子を積む
  // 数値の(1)場合は、左の枝、右の枝、親の右の枝の順で空いているところを探して
  match head {
    2 => Tree::Node(Op::Plus, None, None),
    1 => Tree::Node(Op::Plus, None, None),
    _ => Tree::Node(Op::Plus, None, None),
  };
}
*/

fn build_test(mut tree: TreeNode, depth: i32) -> TreeNode {
  if depth > 4 {
    return tree
  }
  let new_tree_node = TreeNode { left: Tree::Leaf { value: 4 }, right: Tree::Leaf { value: 4 } };
  let new_node = build_test(new_tree_node, depth + 1);
  tree.left = Tree::Node(Box::new(new_node));
  return tree
}

fn calc_tree(tree: &Tree, operands: &Vec<char>, operand_p: &mut usize) -> i32 {
  return match tree {
    Tree::Node(node) => {
      let refnode = &*node;
      let op = *operand_p;
      *operand_p += 1;
      match operands[op] {
        '+' => calc_tree(&refnode.left, &operands, operand_p) + calc_tree(&refnode.right, &operands, operand_p),
        '-' => calc_tree(&refnode.left, &operands, operand_p) - calc_tree(&refnode.right, &operands, operand_p),
        '*' => calc_tree(&refnode.left, &operands, operand_p) * calc_tree(&refnode.right, &operands, operand_p),
        '/' => calc_tree(&refnode.left, &operands, operand_p) / calc_tree(&refnode.right, &operands, operand_p),
        _ => -1
      }
    },
    Tree::Leaf { value: v } => *v,
  }
}

fn format_tree(tree: &Tree) -> String {
  return match tree {
    Tree::Node(node) => {
      let refnode = &*node;
      format!("({} + {})", format_tree(&refnode.left), format_tree(&refnode.right))
    },
    Tree::Leaf { value: v } => format!("{}", *v),
  }
}

fn main(){
  //catalan_tree(vec![].clone(), 0, 0);
  let tree = Tree::Node(Box::new(TreeNode {
    left: Tree::Node(Box::new(TreeNode {
      left: Tree::Leaf { value: 4 },
      right: Tree::Leaf { value: 4 }
    })),
    right: Tree::Node(Box::new(TreeNode {
      left: Tree::Leaf { value: 4 },
      right: Tree::Leaf { value: 4 }
    }))
  }));
  let operands = vec!['+', '+', '+'];
  let mut operand_p = 0;
  let ret = calc_tree(&tree, &operands, &mut operand_p);
  println!("{}", ret);
  println!("{}", format_tree(&tree));
}
