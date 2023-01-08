use std::{rc::Rc, cell::RefCell};

struct TreeNode{
    data : i32,
    left : Option<Rc<RefCell<TreeNode>>>,
    right : Option<Rc<RefCell<TreeNode>>>,
    parent : Option<Rc<RefCell<TreeNode>>>,
}

struct MaxHeap{
    root : Option<Rc<RefCell<TreeNode>>>
}