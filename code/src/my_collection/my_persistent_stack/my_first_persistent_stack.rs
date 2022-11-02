/*
 What is persistent data structure ?
 Non persistent data structures are ephemeral means any update made to the data structure, we loose the previous
 version. For example our implementations in my_stack module.

How persistent data structure looks like 
list1 = A -> B -> C -> D // when time t = 0 : At the beginning of the universe 
list2 = tail(list1) = B -> C -> D //list2 removes element form the beginning stage of the universe
list3 = push(list2, X) = X -> B -> C -> D // list3 adds an element to list2. We can do same thing to list 1 also

How the memory would look ?
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+

The problem : We can't use Box. list2 ownership is a shared ownership. If we use Box, we can't have shared 
ownership, move will happen.

we will use Rc : ReferenceCounted smart pointer

Note : push and pop operations of stack is not be defined in persistent data structure world. 
Therefore, tail is equivalent to pop and prepend is equivalent to push.   

*/

use std::rc::Rc;

 struct Node<'s>{
  data : & 's String,
  next : Option<Rc<Node<'s>>>,
}

pub struct MyFirstPersistentStack<'s>{
  head : Option<Rc<Node<'s>>>
}

impl <'s>MyFirstPersistentStack<'s> {

    pub fn new()->Self{
        return MyFirstPersistentStack { head: Option::None };
    }

    pub fn head(& self)->Option<& 's String>{
        // we can if let Option::Some(cur_head) = & self.head
        // but we can't doe ' if let Option::Some(cur_head) = self.head '. This is because
        // head would be uninitialized and ownership is transferred to cur_head if we do that.  
        if let Option::Some(cur_head) = self.head.as_ref(){
            return Option::Some(cur_head.data);
        }else{
            return Option::None;
        }
    }
    pub fn prepend(& self, element : & 's String)->MyFirstPersistentStack<'s>{
        let new_node:Node<'s> = Node{
            data : element,
            next : self.head.clone(),
        };
        let new_stack : MyFirstPersistentStack<'s> = MyFirstPersistentStack { head: Option::Some(Rc::new(new_node)) };
        return new_stack;
    }

    pub fn tail(& self)->MyFirstPersistentStack<'s>{
        let temp = self.head.as_ref();
        if let Option::Some(next_temp) = temp{
            let new_stack : MyFirstPersistentStack<'s> = MyFirstPersistentStack { 
                head : next_temp.next.clone(),
            };
            return new_stack;
        }else{
            let new_stack : MyFirstPersistentStack<'s> = MyFirstPersistentStack { 
                head : Option::None,
            };
            return new_stack;
        }
        
    }
    
}
//How can we verify our implementation. We can verify using an iterator
/*
How to implement an iterator ? Each instance of iterator maintains index of the current element during iteration.
Every time we call iterator() method we get a new Instance of Iterator object. New instance of the iterator always
points the head of the collection. We have to do exactly same here.
In Java, a Iterator interface is implemented in a static inner class.
*/

//java equivalent of static inner class. next implement the iterator trait
pub struct MyPersistentStackIter<'i>{
 cur_ptr : Option<& 'i Rc<Node<'i>>>
}

impl <'i> Iterator for MyPersistentStackIter<'i>{
    type Item =  & 'i String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Option::Some( cur) = std::mem::replace(& mut self.cur_ptr, Option::None){
            let ret_val = Option::Some(cur.data);
            self.cur_ptr = cur.next.as_ref(); //we can use & too &cur.next
            return ret_val;
        }else{
            return Option::None;
        }
    }
}

#[test]
pub fn test_prepend(){
    let s1:String = String::from("A");
    let s2:String = String::from("B");

    let s3:String = String::from("C");

    let mut stack_1 : MyFirstPersistentStack = MyFirstPersistentStack::new().prepend(& s1).prepend(&s2);
    assert_eq!(*(stack_1.head().unwrap()),s2);

    let mut stack_2 = stack_1.tail();
    assert_eq!(*(stack_2.head().unwrap()),s1);
    let mut stack_3 = stack_2.tail();
    assert_eq!(stack_3.head(),Option::None);
    let mut stack_4 = stack_2.prepend(& s3);
    assert_eq!(*(stack_4.head().unwrap()),s3);
}
    