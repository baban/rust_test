// `4 4 4 4` にカッコと（+, -, /, ÷) を組み合わせて 1 〜 10 の数値を生成せよ

use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;
use std::rc::Rc;

struct TreeNode {
  left: Tree,
  right: Tree
}

enum Tree {
  Node(Box<TreeNode>),
  Leaf { value: i32 },
  Empty
}


struct RefTreeNode {
  left: RefTree,
  right: RefTree
}

enum RefTree {
  Node(Rc<RefCell<RefTreeNode>>),
  Leaf { value: i32 },
  Empty
}

fn is_finish(op_cnt: i32, n_cnt: i32) -> bool {
  op_cnt == 3 && n_cnt == 3
}

fn catalan_path(mut path: Vec<i32>, op_cnt: i32, n_cnt: i32){
  if is_finish(op_cnt, n_cnt){
    // println!("point : {} {}", op_cnt, n_cnt);
    
    path.push(1);

    print!("path : ");
    for d in path.clone() { print!("{} ", d) }
    println!("");
    
    // TODO: Treeを完成させる
    // build_tree(path);
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
    catalan_path(left, op_cnt + 1, n_cnt)
  }
  // 数字を積む
  if op_cnt > n_cnt {
    let mut up = path.clone();
    up.push(1);
    catalan_path(up, op_cnt, n_cnt + 1)
  }
}

fn path_to_ref_tree_node(path: Vec<i32>) -> Option<Rc<RefCell<RefTreeNode>>> {
  // pathから要素を一個取り出す
  // nodeならstackに積む
  // leafならstackからnodeをpopしてnodeのrightかleft空いている方に紐付ける
  // nodeの片方しか埋まっていないときはstackにnodeを戻す
  // nodeの両方が埋まったらstackに戻さない
  // pathの最後までこの処理を行ったら、最後に持っているnodeがtreeのroot
  let mut stack = Vec::<Rc<RefCell<RefTreeNode>>>::new();
  for pnt in path {
    match pnt {
      // node
      2 => {
        let latest_node = stack.pop();
        if let Some(boxed_parent_node) = latest_node {
          let cloned_boxed_parent_node = boxed_parent_node.clone();
          let mut parent_node = (*cloned_boxed_parent_node).borrow_mut();
          match parent_node.left {
             RefTree::Empty => {
               let new_tree_node = Rc::new(RefCell::new(RefTreeNode {
                 left: RefTree::Empty,
                 right: RefTree::Empty
               }));
               parent_node.left = RefTree::Node(new_tree_node.clone());
               stack.push(boxed_parent_node);
               stack.push(new_tree_node.clone());
             },
             _ => {
               let new_tree_node = Rc::new(RefCell::new(RefTreeNode {
                 left: RefTree::Empty,
                 right: RefTree::Empty
               }));
               parent_node.left = RefTree::Node(new_tree_node.clone());
               stack.push(boxed_parent_node);
               stack.push(new_tree_node.clone());
             },
          }
        } else {
          // 最初のnodeはstackが空なのでココが呼ばれる
          let new_tree_node = Rc::new(RefCell::new(RefTreeNode {
            left: RefTree::Empty,
            right: RefTree::Empty
          }));
          stack.push(new_tree_node);
        }
        println!("stack length : {}", stack.len());
      },
      // leaf 
      1 => {
        let latest_node = stack.pop();
        if let Some(boxed_parent_node) = latest_node {
          let cloned_boxed_parent_node = boxed_parent_node.clone();
          let mut parent_node = (*cloned_boxed_parent_node).borrow_mut();
           match parent_node.left {
             RefTree::Empty => {
               parent_node.left = RefTree::Leaf { value: 4 };
               stack.push(boxed_parent_node);
             },
             _ => {
               parent_node.right = RefTree::Leaf { value: 4 };
             },
          }
          println!("stack length : {}", stack.len());
        }
      },
      _ =>{},
    };
  }
  stack.pop()
}

fn translate_tree( ref_tree_node: &RefTree ) -> Tree {
  match ref_tree_node {
    RefTree::Node(rc_ref_tree_node) => {
      let ref_tree_node = &*(*rc_ref_tree_node).borrow();
      return Tree::Node(Box::new(TreeNode {
        left: translate_tree( &ref_tree_node.left ),
        right: translate_tree( &ref_tree_node.left )
      }))
    }
    _ => {
      return Tree::Leaf { value: 4 }
    }
  }
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
        '/' => {
          let a = calc_tree(&refnode.left, &operands, operand_p);
          let b = calc_tree(&refnode.right, &operands, operand_p);
          if a != 0 && b != 0 {
            a / b
          } else {
            -10000
          }
        },
        '%' => {
          let a = calc_tree(&refnode.left, &operands, operand_p);
          let b = calc_tree(&refnode.right, &operands, operand_p);
          if a != 0 && b != 0 {
            a % b
          } else {
            -10000
          }
        },
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
    Tree::Empty => format!("?"),
  }
}

fn build_operand_table(ops: Vec<char>, depth: i32, operand_table: &mut Vec<Vec<char>>){
  if depth == 3 {
    operand_table.push(ops);
    return;
  }
  for op in vec!['+', '-', '*', '/', '%'] {
    let mut clone_ops = ops.clone();
    clone_ops.push(op);
    build_operand_table(clone_ops, depth + 1, &mut *operand_table);
  }
}


fn main(){
  // catalan_path(vec![].clone(), 0, 0);
  let path = vec![2, 2, 2, 1, 1, 1];
  let optioned_ref_tree_node = path_to_ref_tree_node(path);
  if let Some(rc_ref_tree_node) = optioned_ref_tree_node {
    let tree = translate_tree( &RefTree::Node(rc_ref_tree_node) );
    // 式表示
    // let mut operand_p = 0;
    // println!("formula : {}", format_tree(&tree, &vec!['+', '+', '+'], &mut operand_p));
  } else {

  }
  /*
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
  let mut operand_table: Vec<Vec<char>> = vec![];
  build_operand_table(vec![], 0, &mut operand_table);
  for operands in operand_table {
    // 式表示
    let mut operand_p2 = 0;
    println!("formula : {}", format_tree(&tree, &operands, &mut operand_p2));
    // 計算
    let mut operand_p = 0;
    println!("number : {}", calc_tree(&tree, &operands, &mut operand_p));
  }
  */
}
