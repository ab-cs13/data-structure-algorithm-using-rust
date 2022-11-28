/*
 We are going to create a simple binary tree. Each insertion will insert node recursively { Root->Left->Right }
*/

use std::{rc::Rc, cell::RefCell};

struct TreeNode{
    data : i32,
    left : Option<Rc<RefCell<TreeNode>>>,
    right : Option<Rc<RefCell<TreeNode>>>,
    parent : Option<Rc<RefCell<TreeNode>>> 
}
/**
 * Each insertion will insert node recursively following the pattern { Root->Left->Right }. The insertion pattern is equivalent 
 * to BFS traversal of binary tree. 
 */

struct MyFirstBinaryTree{
    root : Option<Rc<RefCell<TreeNode>>>,
}

impl MyFirstBinaryTree{
    fn new()->Self{
        return MyFirstBinaryTree { root: Option::None };
    }
    fn insert(& mut self, element:i32){
        let mut new_node   = Rc::new(RefCell::new(
             TreeNode { 
                data: element, 
                left: Option::None, 
                right: Option::None, 
                parent: Option::None 
            }));
        if self.root.is_none(){
            self.root = Option::Some(new_node.clone());
        }else{

        }
    }
}

#[test]
fn test_01(){

}