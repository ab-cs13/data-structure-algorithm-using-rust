/*
In this implementation of stack we are going to use Option enum rather than creating our own enum.
We are not going to spend time in explaining things in details. We have already done that in my_first_stack
implementation. But, we are going to reiterate important concepts briefly. 
*/

/*
Node of the stack. We don't want ownership of data. As we are accepting reference we need a lifetime for the reference.
lifetime of & T should be less than equal to life time of Node variable 
*/
struct Node<'n,T>{
    data : & 'n T,
    //we can't have Option<Node>. Because Option<Node> is a recursive type definition. struct gets memory from stack.
    //During compile time Rust figure outs memory required for the stack. With recursive definition Rust can't calculate
    //the require memory. Box allocates memory from heap and hold the pointer. It is a smart pointer.
    //Therefore, for Rust Box<Node> will be equal size of smart pointer = pointer size + some additional information (mostly size)
    //  
    next : Option<Box<Node<'n,T>>>,
}
/**
 * A simple stack implementation.
 */
pub struct MySecondStack <'s,T>{
    head : Option<Box<Node<'s,T>>>,
}

impl <'s,T> MySecondStack <'s,T>{
    /**
     * Creates an empty stack
     */
    pub fn new()->Self{
        MySecondStack { head: Option::None }
    }
    /**
     * Add a new elemnt at head position
     */
    pub fn push(& mut self, input:& 's T){
        let mut new_node:Box<Node<'s,T>>  = Box::new(Node{
            data:input,
            next : Option::None,
        });
        let prev_head = std::mem::replace(& mut self.head, Option::None);
        new_node.next = prev_head;
        self.head = Option::Some(new_node);
    }
    /**
     * Returns the element of the head and removes the head
     */
    pub fn pop(& mut self)->Option<& 's T>{
        //can't do if let Option::Some(temp) = self.head because ownership of self.head will be transferred / moved to temp
        //and head become uninitialized causing compilation failure. we need to put something in head when we want the content 
        //of head. Therefore std::meme::replace is used to replace to put None and return the content of head  
        if let Option::Some(temp) = std::mem::replace(& mut self.head,Option::None){
            let return_val : & 's T = temp.data;
            self.head = temp.next;
            return Option::Some(return_val);
        }else{
            return Option::None;
        }
    } 

    /**
     * Returns the element from the head without removing the head.
     */
    pub fn peek(& self)->Option<& 's T>{
        if let Option::Some(temp) = & self.head{
            return Option::Some(temp.data);
        }
        Option::None
    }
}

/*impl  <'s,T> Drop for MySecondStack<'s, T>{
    fn drop(&mut self) {
       
    }
} */   

#[test]
pub fn test_push_pop(){
    let s1:String = String::from("A");
    let s2:String = String::from("B");
    let s3:String = String::from("C");
    let mut stack : MySecondStack<String> = MySecondStack::new();
    stack.push(& s1);
    stack.push(& s2);
    stack.push(& s3);

    assert_eq!(stack.peek().unwrap(),& s3);
    assert_eq!(stack.pop().unwrap(),& s3);
    assert_eq!(stack.pop().unwrap(),& s2);
    assert_eq!(stack.peek().unwrap(),& s1);
    assert_eq!(stack.pop().unwrap(),& s1);
    assert_eq!(stack.pop(),Option::None);
    assert_eq!(stack.peek(),Option::None);
}