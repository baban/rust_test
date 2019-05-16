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
  Leaf { value: i32 },
  Empty
}

fn is_finish(op_cnt: i32, n_cnt: i32) -> bool {
  op_cnt == 3 && n_cnt == 3
}

fn catalan_tree(mut path: Vec<i32>, op_cnt: i32, n_cnt: i32){
  if is_finish(op_cnt, n_cnt){
    // println!("point : {} {}", op_cnt, n_cnt);
    
    path.push(1);

    print!("path : ");
    for d in path.clone() { print!("{} ", d) }
    println!("");
    
    // TODO: Treeを完成させる
    build_tree(path);
    // TODO: 10になるか検算する
    // let ret = calc_tree(&tree, &operands, &mut operand_p);
    // TODO: 10になったらtreeの中身を印字
    // if 1 <= ret && ret <= 10 {
    //  println!("number : {}", ret);
    //  println!("formula : {}", format_tree(&tree, &operands, &mut operand_p2));
    // }
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

fn build_tree(path: Vec<i32>) {
  // pathから要素を一個取り出す
  // nodeならstackに積む
  // leafならstackからnodeをpopしてnodeのrightかleft空いている方に紐付ける
  // nodeの片方しか埋まっていないときはstackにnodeを戻す
  // nodeの両方が埋まったらstackに戻さない
  // pathの最後までこの処理を行ったら、最後に持っているnodeがtreeのroot
  let mut stack = Vec::<Tree>::new();
  
  for pnt in path {
    match pnt {
      // node
      2 => {
        let latest_node = stack.pop();
        // 最初のnodeはstackが空
        let new_tree_node = TreeNode { left: Tree::Empty, right: Tree::Empty };
        let new_node = Tree::Node(Box::new(new_tree_node));
        if let Some(real_latest_node) = latest_node {
          // stack.push(real_latest_node);
          if let Tree::Node(boxed_tree_node) = real_latest_node {
            let mut tree_node = *boxed_tree_node;
            tree_node.left = new_node;
            // stack.push(new_node);
          }
        }
      },
      // leaf 
      1 => {
        let latest_node = stack.pop();
        if let Some(real_latest_node) = latest_node {
          if let Tree::Node(boxed_tree_node) = real_latest_node {
            let mut tree_node = *boxed_tree_node;
            let new_leaf = Tree::Leaf { value: 4 };
            /*
            if tree_node.left == Tree::Empty {
              tree_node.left = new_leaf;
              // stack.push(real_latest_node);
            } else {
              tree_node.right = new_leaf;
            };
            if tree_node.left != Tree::Empty && tree_node.right != Tree::Empty {

            }
            */
          }
        }
        // stack.push(real_latest_node);
      },
      _ =>{},
    };
  }
  // let one_node = stack.pop(); // 最後に1個だけ残されたnodeがroot
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
        _ => -10000
      }
    },
    Tree::Leaf { value: v } => *v,
    Tree::Empty => -10000,
  }
}

fn format_tree(tree: &Tree, operands: &Vec<char>, operand_p: &mut usize) -> String {
  return match tree {
    Tree::Node(node) => {
      let refnode = &*node;
      let op = operands[*operand_p];
      *operand_p += 1;
      format!("({} {} {})", format_tree(&refnode.left, &operands, operand_p), op, format_tree(&refnode.right, &operands, operand_p))
    },
    Tree::Leaf { value: v } => format!("{}", *v),
    Tree::Empty => format!(""),
  }
}

fn main(){
  catalan_tree(vec![].clone(), 0, 0);
  let path = vec![2,2,2,1,1,1,1];
  build_tree(path);
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
  let operands = vec!['-', '+', '/'];
  // 計算
  let mut operand_p = 0;
  println!("number : {}", calc_tree(&tree, &operands, &mut operand_p));
  // 式表示
  let mut operand_p2 = 0;
  println!("formula : {}", format_tree(&tree, &operands, &mut operand_p2));
}
