/*
┌─────────────────────────────────┐
│                                 │
│Pre Order Binary Tree Traversal  │
│                                 │
└─────────────────────────────────┘
Pre order tree traversal is also a variant of 
root->left->right
*/

use std::{rc::Rc, cell::RefCell, collections::HashSet};

//TreeNode_02 for pre-order traversal
struct TreeNode<'s>{
  data : & 's String,
  left : Option<Rc<RefCell<TreeNode<'s>>>>,
  right : Option<Rc<RefCell<TreeNode<'s>>>> 
}
impl<'t> TreeNode<'t>{
    fn new(element : & 't String,left_node:Option<TreeNode<'t>>,right_node:Option<TreeNode<'t>>)->Self{
        let mut temp = TreeNode { data: element, left: Option::None, right: Option::None };
        if let Option::Some(left) = left_node{
            temp.left = Option::Some(Rc::new(RefCell::new(left)));
        }
        if let Option::Some(right) = right_node{
            temp.right = Option::Some(Rc::new(RefCell::new(right)));
        }
        return temp;
    }
    fn unique_id(node : & Rc<RefCell<TreeNode<'t>>>)->usize{
        let raw_ptr = node.as_ptr() as *const TreeNode;
        return raw_ptr as usize;
    }
}
struct BinaryTree<'s>{
    root : Option<Rc<RefCell<TreeNode<'s>>>>
}
impl <'s>BinaryTree<'s>{

    //root->left->right
    fn pre_order_traversal(& self)->Vec<& 's String>{
        if self.root.is_none(){
            return Vec::new();
        }else{
            let mut nodes : Vec<& 's String> = Vec::new();
            self.do_pre_order(self.root.as_ref().unwrap(), &mut nodes);
            return nodes;
        }
    }
    fn do_pre_order(& self, temp:& Rc<RefCell<TreeNode<'s>>>, nodes:  & mut Vec<& 's String>){
        
        nodes.insert(nodes.len(), temp.borrow().data);
        if temp.borrow().left.is_some(){
            self.do_pre_order(temp.borrow().left.as_ref().unwrap(), nodes);
        }
        if temp.borrow().right.is_some(){
            self.do_pre_order(temp.borrow().right.as_ref().unwrap(), nodes);
        }
    }
}

impl <'s> Drop for BinaryTree<'s>{
    fn drop(&mut self) {
        if self.root.is_some(){
            // we will perform BFS traversal and manually decrease the reference count to 0.
            // We are going to use vec (memory allocated from heap) to perform BFS. Therefore, we will not face stack
            // overflow problem.
            let mut queue : Vec<Rc<RefCell<TreeNode<'s>>>> = Vec::new();
            queue.insert(0, self.root.clone().unwrap());
            while queue.len() > 0{
                //remove the 1st element
                let temp = queue.remove(0);
                eprintln!("Dropping : {}",temp.borrow().data);
                if temp.borrow().left.is_some(){
                    queue.insert(queue.len(), temp.borrow().left.clone().unwrap());
                    temp.borrow_mut().left = Option::None;
                }
                if temp.borrow().right.is_some(){
                    queue.insert(queue.len(), temp.borrow().right.clone().unwrap());
                    temp.borrow_mut().right = Option::None;
                }
            }

        }//else nothing to do
    }
}
struct Iter<'s>{
    stack : Vec<Rc<RefCell<TreeNode<'s>>>>,
    visited : HashSet<usize>
} 

impl<'s> Iter<'s>{
    fn new()->Self{
        let mut iter =  Iter { stack: Vec::new(), visited: HashSet::new() };

        
        return iter;
    }
}

impl<'s> Iterator for Iter<'s>{
    type Item = & 's String;

    fn next(&mut self) -> Option<Self::Item> {

        todo!()
    }
}


#[test]
fn test_01(){
 let s1 = String::from("1");
 let s2 = String::from("2");
 let s5 = String::from("5");
 let s3 = String::from("3");
 let s4 = String::from("4");
 let s6 = String::from("6");
 let s7 = String::from("7");

 let mut binary_tree:BinaryTree = BinaryTree { root: Option::Some(Rc::new(RefCell::new(TreeNode{
  data : & s1,
  left : Option::None,
  right : Option::None,
 }))) };

let mut s2_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s2,
    left : Option::None,
    right : Option::None,
   })));
   binary_tree.root.as_ref().unwrap().borrow_mut().left = s2_node.clone();

/*
TODO: why below code won't work ??
binary_tree.root.as_ref().unwrap().borrow().left.unwrap().borrow_mut().left = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s2,
    left : Option::None,
    right : Option::None,
   })));   */

   let mut s3_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s3,
    left : Option::None,
    right : Option::None,
   })));
   s2_node.unwrap().borrow_mut().left = s3_node.clone();

   let mut s5_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s5,
    left : Option::None,
    right : Option::None,
   })));

   binary_tree.root.as_ref().unwrap().borrow_mut().right = s5_node.clone();
   let mut s6_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s6,
    left : Option::None,
    right : Option::None,
   })));

   s5_node.as_ref().unwrap().borrow_mut().left = s6_node.clone();

   let nodes:Vec<& String> = binary_tree.pre_order_traversal();
   //1 → 2 → 3  → 5 → 6 
   for s in nodes{
    eprintln!("element : {}", s);
   }
  
}


/**
 *               Tree is
 *                 1
 *               /   \
 *              /     \
 *             2       3 
 *            / \     /  \
 *           /   \   /    \
 *          5    6  7      8
 * Pre Order: 1 → 2 → 3 → 4 → 5 → 6 → 7
 */  
#[test]
fn test_02(){
 let s1 = String::from("1");
 let s2 = String::from("2");
 let s5 = String::from("5");
 let s3 = String::from("3");
 let s4 = String::from("4");
 let s6 = String::from("6");
 let s7 = String::from("7");

 let mut binary_tree:BinaryTree = BinaryTree { root: Option::Some(Rc::new(RefCell::new(TreeNode{
  data : & s1,
  left : Option::None,
  right : Option::None,
 }))) };

let mut s2_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s2,
    left : Option::None,
    right : Option::None,
   })));
   binary_tree.root.as_ref().unwrap().borrow_mut().left = s2_node.clone();


   let mut s3_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s3,
    left : Option::None,
    right : Option::None,
   })));
   s2_node.as_ref().unwrap().borrow_mut().left = s3_node.clone();

   let mut s4_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s4,
    left : Option::None,
    right : Option::None,
   })));
   s2_node.as_ref().unwrap().borrow_mut().right = s4_node.clone();

   let mut s5_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s5,
    left : Option::None,
    right : Option::None,
   })));
   binary_tree.root.as_ref().unwrap().borrow_mut().right = s5_node.clone();

   let mut s6_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s6,
    left : Option::None,
    right : Option::None,
   })));
   s5_node.as_ref().unwrap().borrow_mut().left = s6_node.clone();

   let mut s7_node = Option::Some(Rc::new(RefCell::new(TreeNode{
    data : & s7,
    left : Option::None,
    right : Option::None,
   })));
   s5_node.as_ref().unwrap().borrow_mut().right = s7_node.clone();

   let nodes:Vec<& String> = binary_tree.pre_order_traversal();
   //1 → 2 → 3  → 5 → 6 
   for s in nodes{
    eprintln!("element : {}", s);
   }
  
}


