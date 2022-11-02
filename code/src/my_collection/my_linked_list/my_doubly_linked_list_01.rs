/*
We are going to implement doubly linked list.
    ______     _______      _______
   |   1  |-> |     2 | -> |     3 |
   |______|<- |_______| <- |_______|

   Every node is referred by two pointer i.e. 'next' and 'prev'. For example node 2 is referred by 'next pointer' of
   1 and 'prev pointer' of 3. Therefore node 2 is eligible for drop only when both node 1 and node 2 is dropped.
   Since we need multiple reference to point a given node, the node must be wrapped inside a Rc smart pointer.
   
   So our 1st tool : Rc smart pointer
   ----------------------------------

   Since Rust doesn't have a null value our 2nd tool is Option. And template would look something like Option<Rc>
   --------------------------------------------------------------------------------------------------------------

   We also need head and tail to hold the address of 1st node and last node respectively to perform insertion in O(1) time.

   Anything else ? Let's first create our Node template using concepts till now
 
   struct Node{
    data : i32,
    next : Option<Rc<Node>>,
    prev : Option<Rc<Node>>,
   }
   
struct MyDLL{ //MyDoublyLinkedList
  head : Option<Rc<Node>>,
  tail : Option<Rc<Node>>,
}

Till now all our insertion and deletion happening at head. For insertion we have used below code snippet :

let prev_head = std::mem::replace(& mut self.head, Option::None);
new_node.next = prev_head;
self.head = Option::Some(new_node);

If we look closely we put Option::None in head and latter assign head to our new node. We didn't have any
requirement to mutate the inner content of head. But, this logic won't work for our doubly linked list.
Because we have to mutate the 'next' and 'prev' pointers which are the inner content of head and tails.

That's why we need 'RefCell smart pointer' which will allow us to mutate the inner content. This our 3rd tool
RefCell follow the same principle:
single and multiple readers are exclusive to each other. But, the principle is che ked during runtime rather than
compile time. And if rule is breached, program panics during runtime. 
-------------------------------------------------------------------------------------------------------------

I hope with these tools we can create a basic doubly linked list. Let's give it a go. 

Another most important tool that is going to help us a lot is the clone() method of Rc smart pointer. When we invoke the
clone method Rc creates a new pointer in stack which holds the address of the heap allocation and increases the reference 
count of. when the pointer goes out of scope, the reference count is decreased. When reference count becomes zero, the 
heap content is dropped.

Clone method of Option. When we call the clone method method of Option, the clone of the content is called and wrapped
inside Option and returned to the caller.

   

 
*/

use std::{rc::Rc, cell::RefCell};

struct Node{
    data : i32,
    //since we have to mutate the inner content Node must wrapped by RefCell
    next : Option<Rc<RefCell<Node>>>,
    prev : Option<Rc<RefCell<Node>>>,
}

struct MyDLL{ //MyDoublyLinkedList
  head : Option<Rc<RefCell<Node>>>,
  tail : Option<Rc<RefCell<Node>>>,
  size:i32,
}

impl MyDLL{
    /**
     * Creates a new doubly linked list.
     */
    fn new()->Self{
        return MyDLL { head: Option::None, tail: Option::None , size:0};
    }
    /**
     * Appends an element
     */
    fn append(& mut self, element:i32){
        if let Option::Some(cur_tail) = std::mem::replace(& mut self.tail,Option::None){ //if we have something in tail
            let new_node : Rc<RefCell<Node>> = Rc::new(RefCell::new(
                Node { data: element, next: Option::None, prev: Option::Some(cur_tail.clone()) }
            ));
            //now we need the smart pointer RefCell to add the new_node to cur_tail
            cur_tail.borrow_mut().next = Option::Some(new_node.clone());
            //Now update the tail to point to new_node
            self.tail = Option::Some(new_node.clone());

        }else{
            let new_node : Rc<RefCell<Node>> = Rc::new(RefCell::new(
                Node { data: element, next: Option::None, prev: Option::None }
            ));
            self.head = Option::Some(new_node.clone());
            self.tail = Option::Some(new_node.clone());
        }
        self.size = self.size+1;
    }

    /**
     * Adds an element at the head
     */
    fn add_at_head(& mut self, element : i32){
      if let Option::Some(cur_head) = std::mem::replace(& mut self.head, Option::None){
        let new_node : Rc<RefCell<Node>> = Rc::new(RefCell::new(
            Node { data: element, next: Option::Some(cur_head.clone()), prev: Option::None }
        ));
        self.head = Option::Some(new_node.clone());

      }else{
        let new_node : Rc<RefCell<Node>> = Rc::new(RefCell::new(
            Node { data: element, next: Option::None, prev: Option::None }
        ));
        self.head = Option::Some(new_node.clone());
        self.tail = Option::Some(new_node.clone());
      }
      self.size = self.size+1;
    }
    
    /**
     * Add an element at a given position
     */
    fn add_at(& mut self,element:i32,position:i32){
        if position > self.size{
            panic!("Out of index");
        }else if position == self.size{
         self.append(element);
        }else if position==0 {
            self.add_at_head(element);
        }
        else{
            //The clone method of Rc increases the reference count 
            let mut temp = self.head.clone().unwrap(); // pointed to head
            let mut i:i32 = 0;
            //we need to loop till (position-1)th place
            while i < position-1{
             let t=temp.clone();
             temp = t.borrow().next.as_ref().unwrap().clone();
             i = i+1;  
            }
            //temp is at position-1 place
            let new_node = Rc::new(RefCell::new(Node{
                data : element,
                //clone of option clones the content of the option. Here clone of Rc is called
                next : temp.borrow().next.clone(),
                prev : Option::Some(temp.clone()),
            }));
            //tmep.next.prev should point to new_node
            let mut mut_temp = temp.borrow_mut();
            mut_temp.next.as_ref().unwrap().borrow_mut().prev = Option::Some(new_node.clone());
            //Now temp.next point to new node.
            mut_temp.next = Option::Some(new_node.clone());
        }
    }
    fn iter_from_front(& self)->IterFromFront{
        return IterFromFront { cur_ptr: self.head.clone() };
    }

    fn delete(& mut self,index:i32){
        if index < 0{
            panic!("supplied index is negative");
        }else if index >= self.size {
            panic!("Out of bound index. index:{}, size:{}",index, self.size);
        }else if self.size == 0{
            let ret_val:i32 = self.head.as_ref().unwrap().borrow().data;
            self.head = Option::None;
            self.tail = Option::None;
            //return ret_val; 
        }else{
            let mut temp = self.head.clone(); 
            let mut i : i32 = 0;
            while i < index-1{
                let temp_1 = temp.clone();
                //let k=temp_1.;
                temp = temp_1.unwrap().borrow().next.clone();
                i=i+1;
            }

            
        } 
    }
}

struct IterFromFront{
    cur_ptr : Option<Rc<RefCell<Node>>>,
}
impl Iterator for IterFromFront{
    type  Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        
      if let Option::Some(t) = self.cur_ptr.clone(){
        let return_val = Option::Some(t.borrow().data);
        self.cur_ptr = t.clone().borrow().next.clone();
        return return_val;
      }else{
        return Option::None;
      }

    }

    
}


#[test]
fn test_append(){
    let mut ll:MyDLL = MyDLL::new();
    ll.append(0);
    ll.append(1);
    ll.append(2);
    ll.append(3);
    let mut it:IterFromFront = ll.iter_from_front();
    let mut i:i32 = 0;
    while let Option::Some(element) = it.next(){
      assert_eq!(i,element);
      i = i+1;
    } 
}

#[test]
fn test_add_at_head(){
    let mut ll:MyDLL = MyDLL::new();
    ll.add_at_head(3);
    ll.add_at_head(2);
    ll.add_at_head(1);
    ll.add_at_head(0);
    let mut it:IterFromFront = ll.iter_from_front();
    let mut i:i32 = 0;
    while let Option::Some(element) = it.next(){
      assert_eq!(i,element);
      i = i+1;
    } 
}
#[test]
fn test_add_at(){
    let mut ll:MyDLL = MyDLL::new();
    ll.append(0);
    ll.append(2);
    ll.append(3);
    ll.add_at(1, 1);
    let mut it:IterFromFront = ll.iter_from_front();
    let mut i:i32 = 0;
    while let Option::Some(element) = it.next(){
      assert_eq!(i,element);
      i = i+1;
    } 
}