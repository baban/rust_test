use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::Ref;

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
  Node(Box<RefTreeNode>),
  Leaf { value: i32 },
  Empty
}

fn main(){
  let a = Rc::new(RefCell::new(RefTreeNode { left: RefTree::Empty, right: RefTree::Empty }));
  let mut b = Vec::<Ref<RefTreeNode>>::new();
  let mut c = RefTreeNode { left: RefTree::Empty, right: RefTree::Empty };
  b.push(a.borrow());
  let d: Ref<RefTreeNode> = a.borrow();
  let e = RefTreeNode { left: RefTree::Empty, right: RefTree::Empty };
  //let d = Rc::new(a.borrow());
  c.left = RefTree::Empty;
  //c.left = RefTree::Node(Box::new(*d));
  //c.left = RefTree::Node(Box::new(d));

  //println!("{:?}", a);
  //println!("{:?}", a.borrow());
  // b.push(a.borrow());
  //println!("{:?}", *(a.borrow()));
  //short_lifetime(b);
}
