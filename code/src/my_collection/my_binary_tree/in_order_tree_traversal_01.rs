/*
┌─────────────────────────────────┐
│                                 │
│In Order Traversal of Binary Tree│
│                                 │
└─────────────────────────────────┘

 In order tree traversal is a form of DFS traversal of a binary tree. 
 Traversal is performed in Left->Root->Right manner.

        A
       / \       => B,D,A,C,E
      B   C
       \    \
        D    E


We need to implement Drop trait. Why ? Refer nots in post_order_tree_traversal_01.rs

*/          

use std::{rc::Rc, cell::RefCell, collections::HashSet};

struct TreeNode<'t>{
    data : & 't String,
    left : Option<Rc<RefCell<TreeNode<'t>>>>,
    right : Option<Rc<RefCell<TreeNode<'t>>>>,
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
}

struct BinaryTree<'a>{
    root : Option<Rc<RefCell<TreeNode<'a>>>>,
}

impl <'a>BinaryTree<'a>{
    
    /**
     * Performs the in-order traversal and inserts those in the vector and returns the vector.
     */
    fn recursive_in_order(& self)->Vec<& 'a String>{
        if self.root.is_none(){
            return Vec::new(); //just return an empty vector
        }else{
            let mut nodes : Vec<& 'a String> = Vec::new();
            self.do_in_order(self.root.as_ref().unwrap(),&mut nodes);
            return nodes;
        }
    }
    // Left->Root->Right
    fn do_in_order(& self,temp_node:& Rc<RefCell<TreeNode<'a>>>, nodes: & mut Vec<& 'a String>){
        if temp_node.borrow().left.is_some(){
            self.do_in_order(temp_node.borrow().left.as_ref().unwrap(), nodes);
        }
        nodes.insert(nodes.len(), temp_node.borrow().data);
        if temp_node.borrow().right.is_some(){
            self.do_in_order(temp_node.borrow().right.as_ref().unwrap(), nodes);
        }
    }

    fn get_in_order_iter(&  self)->InOrderIter<'a>{
        return InOrderIter ::new(self.root.clone().unwrap());
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

struct InOrderIter<'s>{
    stack : Vec< Rc<RefCell<TreeNode<'s>>>>,
    visited_nodes : HashSet<usize>

}
impl<'s> InOrderIter<'s>{
    fn new(root_ref :Rc<RefCell<TreeNode<'s>>> )->Self{
        let mut inorder_iter : InOrderIter = InOrderIter { 
            stack: Vec::new(),
            visited_nodes : HashSet::new()
        };
        //just push the root node
        inorder_iter.stack.push(root_ref);
        return inorder_iter;
    }
} 
impl<'s> Iterator for InOrderIter<'s>{
    type Item = & 's String;

    fn next(&mut self) -> Option<Self::Item> {    
       if self.stack.is_empty(){
        return Option::None;
       }else{
         //get the stack top to verify whether any stack_top.left exists or not
         let mut stack_top  = self.stack.last().unwrap().clone();
         
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
            self.stack.push(temp_left.clone());
            stack_top = temp_left.clone();
         }
         //Now we have reached the end of the left child traversal
         //pop the stack top
         let popped_element=self.stack.pop().unwrap();
         //if the popped element has a right child push it inside the stack. 
         if let Option::Some(right_child) = popped_element.borrow().right.clone(){
            self.stack.push(right_child);
         }
         let raw_ptr_of_popped_one = popped_element.as_ptr() as *const TreeNode;
         let mem_address_of_popped = raw_ptr_of_popped_one as usize; 
         self.visited_nodes.insert(mem_address_of_popped); 
         return Option::Some(popped_element.borrow().data);
       } 
    }
    
        
    }


 #[test]
 fn test_1(){
    let a = String::from("A");
    let b = String::from("B");
    let c = String::from("C");
    let d = String::from("D");
    let e = String::from("E");
/*
        A
       / \       => B,D,A,C,E
      B   C
       \    \
        D    E
*/
let node_d = TreeNode::new(&d, Option::None, Option::None);
let node_b = TreeNode::new(&b, Option::None, Option::Some(node_d));
let node_e = TreeNode::new(&e, Option::None, Option::None);
let node_c = TreeNode::new(&c, Option::None, Option::Some(node_e));
let node_a = TreeNode::new(&a, Option::Some(node_b), Option::Some(node_c));

let b_tree = BinaryTree{root : Option::Some(Rc::new(RefCell::new(node_a)))};
let expected_order: [& String; 5] = [&b,&d,&a,&c,&e];
eprintln!("---- recursion ----");
let nodes = b_tree.recursive_in_order();
let mut i=0;
while i < nodes.len(){
    assert_eq!(expected_order[i],*(nodes.get(i).unwrap()));
    eprintln!("element : {}", *(nodes.get(i).unwrap()));
    i = i+1;
}
eprintln!("---- iterator ----");
let mut iter = b_tree.get_in_order_iter();
let mut j=0;
while let Option::Some(temp)=iter.next(){
    assert_eq!(expected_order[j],temp);
    eprintln!("element : {}", *(temp));
    j=j+1;
}

 }