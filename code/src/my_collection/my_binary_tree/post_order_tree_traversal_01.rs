/*
┌─────────────────────────────────┐
│                                 │
|Post Order Binary Tree Traversal │
│                                 │
└─────────────────────────────────┘
Post order binary tree traversal is a form of DFS traversal. Recursively we visit 
Left,Right, Root
Example : 

               A
             /   \
            /     \
           B       C
          / \     / \
         /   \   /   \
        D     E F     G
      
   Post order traversal will generate : D,E,B,F,G,C,A
   
   Here we are going to implement the post order tree traversal algorithm both in recursive and iterator design pattern. 


         A
       / 
      B
       \
        C
       / \
      D   E

      Post order traversal : D,E,C,B,A
   Note on Drop trait implementation
   ---------------------------------
   The question we need to ask : Do we need to implement Drop trait ? Let's go back to our basics. How Drop trait works
   internally ? 
   
   When drop is called on a given node. It internally calls the drop on the left pointer, calls drop on right pointer
   recursively. The recursive call can't be converted to tail recursion and can't be tail call optimized. Why ?

   1. drop called on root
   2. drop called on root.left pointer<TreeNode> which calls drop on root.left.left and so on. 
   3. drop called on root.right and so on

   Because of recursive type definition of the TreeNode, compiler can not convert recursive drop call to loop(
   compiler cannot perform tail call optimization. In tail call optimization compiler converts recursive tail call
   to loop ). All tree nodes are created in heap. During drop call there is a fair possibility that stack won't
   have enough memory to hold the recursive call.  Therefore we have to implement drop trait for BinaryTree.
*/

use std::{rc::Rc, cell::RefCell, collections::HashSet, path::Iter};

struct TreeNode<'s>{
    data : & 's String,
    left : Option<Rc<RefCell<TreeNode<'s>>>>,
    right : Option<Rc<RefCell<TreeNode<'s>>>>,
}
impl <'s> TreeNode<'s>{
    fn new(element : & 's String,left_node:Option<TreeNode<'s>>,right_node:Option<TreeNode<'s>>)->Self{
        let mut temp = TreeNode { data: element, left: Option::None, right: Option::None };
        if let Option::Some(left) = left_node{
            temp.left = Option::Some(Rc::new(RefCell::new(left)));
        }
        if let Option::Some(right) = right_node{
            temp.right = Option::Some(Rc::new(RefCell::new(right)));
        }
        return temp;
    }
    fn unique_id(node : & Rc<RefCell<TreeNode<'s>>>)->usize{
        let raw_ptr = node.as_ptr() as *const TreeNode;
        return raw_ptr as usize;
    }
}

struct BinaryTree<'s>{
    root : Option<Rc<RefCell<TreeNode<'s>>>>,
}
impl<'s> BinaryTree<'s>{


    fn perform_post_order_traversal(& self)->Vec<& 's String>{
        let mut nodes : Vec<& 's String> = Vec::new();
        if self.root.is_some(){
            self.traverse(self.root.as_ref().unwrap(), &mut nodes);
        }
        return nodes;
    }

    fn traverse(& self, temp_node:& Rc<RefCell<TreeNode<'s>>>,nodes : & mut Vec<& 's String>){
        if temp_node.borrow().left.is_some(){
            self.traverse(temp_node.borrow().left.as_ref().unwrap(), nodes);
        }
        //there is no left node. is there any right node
        if temp_node.borrow().right.is_some(){
            self.traverse(temp_node.borrow().right.as_ref().unwrap(), nodes);
        } 
        nodes.insert(nodes.len(), temp_node.borrow().data);
    }

    fn get_iter(&  self)->BinaryTreeIter<'s>{
        let mut temp  = BinaryTreeIter { stack: Vec::new(), visited_nodes: HashSet::new() };
        temp.stack.push(self.root.clone().unwrap());
        return temp;
    }
}
/**
 * In the drop trait implementation, 
 */
impl <'s> Drop for BinaryTree<'s>{
    fn drop(&mut self) {
        if self.root.is_some(){
        //We will do BFS traversal and disconnect parent and child nodes.
        let mut nodes : Vec<Rc<RefCell<TreeNode<'s>>>> = Vec::new();
           nodes.insert(nodes.len(), self.root.as_ref().unwrap().clone());
           while nodes.len() != 0{
            //dequeue the 1st element of nodes vector
            let temp = nodes.remove(0);
            if temp.borrow().left.is_some(){
                nodes.insert(nodes.len(), temp.borrow().left.as_ref().unwrap().clone());
            }
            if temp.borrow().right.is_some(){
                nodes.insert(nodes.len(), temp.borrow().right.as_ref().unwrap().clone());
            }
            temp.borrow_mut().left = Option::None;
            temp.borrow_mut().right = Option::None;
           } 
        }
    }
}

struct BinaryTreeIter<'s>{
    stack : Vec<Rc< RefCell<TreeNode<'s>>>>,
    visited_nodes : HashSet<usize>,
}

impl<'s> BinaryTreeIter<'s>{
    fn push_left_nodes(& mut self){
        let mut stack_top=self.stack.last().unwrap().clone();
        //push the left in the stack if and only if the left child is never visited.
        if stack_top.borrow().left.is_some(){
            let left_raw_ptr = stack_top.borrow().left.as_ref().unwrap().as_ptr() as *const TreeNode;
            let mem_address : usize = left_raw_ptr as usize;
            if ! self.visited_nodes.contains(&mem_address){
                while stack_top.borrow().left.is_some(){
                    let temp_left =  stack_top.borrow().left.clone().unwrap();
                    self.stack.push(temp_left.clone());
                    stack_top = temp_left.clone();
                }
            }
        }
    }
}

impl<'s> Iterator for BinaryTreeIter<'s>{
    type Item = & 's String;

    fn next(&mut self) -> Option<Self::Item> {
        //get the stack top until stack is empty
        if ! self.stack.is_empty(){
            
            //pick the last element
            loop  {
                //pick the stack top
                let stack_top  = self.stack.last().unwrap().clone();
                //if left is there and not visited
                if stack_top.borrow().left.is_some() && 
                                !self.visited_nodes.contains(& TreeNode::unique_id(stack_top.borrow().left.as_ref().unwrap())){
                    self.stack.push(stack_top.borrow().left.clone().unwrap());                
                }else if stack_top.borrow().right.is_some() && //if right is there and not visited
                                !self.visited_nodes.contains(& TreeNode::unique_id(stack_top.borrow().right.as_ref().unwrap())){
                    self.stack.push(stack_top.borrow().right.clone().unwrap());                    
                }else{
                    break;
                }
               
            }
            let node_to_remove = self.stack.pop().unwrap();
            let temp_raw_ptr = node_to_remove.as_ptr() as *const TreeNode;
            let temp_mem_address: usize = temp_raw_ptr as usize;
            self.visited_nodes.insert(temp_mem_address);
            return Option::Some(node_to_remove.borrow().data);
        }else{
            return Option::None;
        }
    }
  
}

#[test]
fn test_1(){
    let a : String = String::from("A");
    let b : String = String::from("B");
    let e : String = String::from("E");

    let mut a_node =TreeNode::new(& a,Option::None,Option::None);
    let mut b_node = TreeNode::new(& b,Option::None,Option::None);
    let  e_node = TreeNode::new(&e,Option::None,Option::None);
    b_node.right = Option::Some(Rc::new(RefCell::new(e_node)));
    a_node.left = Option::Some(Rc::new(RefCell::new(b_node)));
    let b_tree = BinaryTree{root:Option::Some(Rc::new(RefCell::new(a_node)))};
    let nodes=b_tree.perform_post_order_traversal();
    let mut i:usize =0;
    while i < 3{
        eprintln!("element : {}", *(nodes.get(i).unwrap()));
        i = i+1;
    }

}

#[test]
fn test_2(){
/*
    A
    / 
   B
    \
     C
    / \
   D   E

   Post order traversal : D,E,C,B,A
*/

let a = String::from("A");
let b = String::from("B");
let c = String::from("C");
let d = String::from("D");
let e = String::from("E");

let node_e = TreeNode::new(&e,Option::None,Option::None);
let node_d = TreeNode::new(&d,Option::None,Option::None);
let node_c = TreeNode::new(&c,Option::Some(node_d),Option::Some(node_e));
let node_b = TreeNode::new(&b, Option::None, Option::Some(node_c));
let node_a = TreeNode::new(&a,Option::Some(node_b),Option::None);

let  b_tree = BinaryTree{root : Option::Some(Rc::new(RefCell::new(node_a)))};
eprintln!("---- recursion ----");
let nodes = b_tree.perform_post_order_traversal();
let mut i=0;
while i < nodes.len(){
    eprintln!("element : {}", *(nodes.get(i).unwrap()));
    i = i+1;
}
eprintln!("---- iterator ----");
let mut iter = b_tree.get_iter();
while let Option::Some(temp)=iter.next(){
    eprintln!("element : {}", *(temp));
}
}