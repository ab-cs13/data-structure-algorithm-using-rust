/*
 We are going to create a simple binary tree. Each insertion will insert node recursively { Root->Left->Right }. Similar to
 BFS traversal.
*/

use std::{rc::Rc, cell::RefCell, collections::HashSet};

struct TreeNode<'t>{
    data : & 't String,
    left : Option<Rc<RefCell<TreeNode<'t>>>>,
    right : Option<Rc<RefCell<TreeNode<'t>>>>,
    parent : Option<Rc<RefCell<TreeNode<'t>>>> 
}
/**
 * Each insertion will insert node recursively following the pattern { Root->Left->Right }. The insertion pattern is equivalent 
 * to BFS traversal of binary tree. 
 */

struct MyFirstBinaryTree<'a>{
    root : Option<Rc<RefCell<TreeNode<'a>>>>,
    queue_for_insert : Vec<Rc<RefCell<TreeNode<'a>>>>,
}

impl <'a>MyFirstBinaryTree<'a>{
    fn new()->Self{
        return MyFirstBinaryTree { 
            root: Option::None ,
            queue_for_insert:Vec::new()};
    }
    fn insert(& mut self, element:& 'a String){
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
            let queue_top = self.queue_for_insert.get(0).unwrap();
            if queue_top.borrow().left.is_none(){
                queue_top.borrow_mut().left = Option::Some(new_node.clone());
            }else{
                queue_top.borrow_mut().right = Option::Some(new_node.clone());
                self.queue_for_insert.remove(0);
            }
        }
        let  index_to_insert : usize = self.queue_for_insert.len(); 
        self.queue_for_insert.insert(index_to_insert, new_node.clone());
    }

    fn recursive_in_order(& self)->Vec<& 'a String>{
        if self.root.is_none(){
            return Vec::new(); //just return an empty vector
        }else{
            let mut nodes : Vec<& 'a String> = Vec::new();
            self.do_inorder(self.root.as_ref().unwrap(),&mut nodes);
            return nodes;
        }
    }
    fn do_inorder(& self,temp_node:& Rc<RefCell<TreeNode<'a>>>, nodes: & mut Vec<& 'a String>){
        if temp_node.borrow().left.is_none(){
            nodes.insert(nodes.len(), temp_node.borrow().data);
            if temp_node.borrow().right.is_some(){
                self.do_inorder(temp_node.borrow().right.as_ref().unwrap(), nodes);
            }
        }else{
            self.do_inorder(temp_node.borrow().left.as_ref().unwrap(), nodes);
        }

    }
}

struct BinaryTreeInOrderIter<'s>{
    //lifetime of data is >= lifetime of tree node. That's why two lifetime.
    tree_nodes_stack : Vec< Rc<RefCell<TreeNode<'s>>>>,
    visited_nodes : HashSet<usize>

}
impl<'s> BinaryTreeInOrderIter<'s>{
    fn new(root_ref :Rc<RefCell<TreeNode<'s>>> )->Self{
        let mut inorder_iter : BinaryTreeInOrderIter = BinaryTreeInOrderIter { 
            tree_nodes_stack: Vec::new(),
            visited_nodes : HashSet::new()
        };
        //just push the root node
        inorder_iter.tree_nodes_stack.push(root_ref);
        return inorder_iter;
    }
} 
impl<'s> Iterator for BinaryTreeInOrderIter<'s>{
    type Item = & 's String;

    fn next(&mut self) -> Option<Self::Item> {    
       if self.tree_nodes_stack.is_empty(){
        return Option::None;
       }else{
         //get the stack top to verify whether any stack_top.left exists or not
         let mut stack_top  = self.tree_nodes_stack.get(self.tree_nodes_stack.len()-1).unwrap().clone();
         
         //if stack top has left child traverse till there is no left child
         /*
         Note : I have cloned the stack top why ? stack_top is mut. I can't have borrowed immutable stack_top and mut stack_top
         Can't evn borrow borrow_mut (mut and borrow_mut can't coexist in a given scope or lifetime.) When I clone it creates a 
         new variable which is borrowed immutably 
         */
         while let Option::Some(temp_left) = stack_top.clone().borrow().left.clone(){
            //push to stack if not visited if visited just brake the loop and pop it.
            let raw_ptr_temp : * const TreeNode = temp_left.as_ptr() as *const TreeNode;
            let temp_mem_address = raw_ptr_temp as usize;
            if self.visited_nodes.contains(&temp_mem_address){
                break;
            }
            self.tree_nodes_stack.push(temp_left.clone());
            stack_top = temp_left.clone();
         }
         //Now we have reached the end of the left child traversal
         //pop the stack top
         let popped_element=self.tree_nodes_stack.pop().unwrap();
         //if the popped element has a right child push it inside the stack. 
         if let Option::Some(right_child) = popped_element.borrow().right.clone(){
            self.tree_nodes_stack.push(right_child);
         }
         let raw_ptr_of_popped_one = popped_element.as_ptr() as *const TreeNode;
         let mem_address_of_popped = raw_ptr_of_popped_one as usize; 
         self.visited_nodes.insert(mem_address_of_popped); 
         return Option::Some(popped_element.borrow().data);
       } 
    }
    
        
    }


#[test]
fn test_01(){

}