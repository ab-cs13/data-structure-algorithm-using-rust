/*
 We are going to implement a simple stack which accepts only String types. 
 Why specific implementation : learn different variants of iterator. 
*/

struct Node{
    data : String,
    next : Option<Box<Node>>
}
pub struct StringStack{
    head : Option<Box<Node>>
}

/*
  Iterators have some kind of index to hold the current position. To implement our iterator we need similar 
  mechanism. Either we can change our StringStack struct template or we need a wrapper. For wrapper is the only
  solution because we can't return the String we have to return '& String' 
 */
pub struct StringStackIter<'s>{
    //we need to have the reference
    node_ptr: & 's Option<Box<Node>>, 
}

impl StringStack{
    pub fn new()->Self{
        return StringStack { head: Option::None };
    }
    /**
     * Push the string to the stack
     */
    pub fn push(& mut self, input:String){
      let current_head : Option<Box<Node>> = std::mem::replace(& mut self.head, Option::None);
      let new_node : Node = Node { data: input, next: current_head };
      self.head = Option::Some(Box::new(new_node));  
    }

    /**
     * Pops the String there in the top of the stack. If empty returns None
     */
     pub fn pop(& mut self)->Option<String>{
        let  current_head : Option<Box<Node>> = std::mem::replace(& mut self.head,Option::None);
        if let Option::Some(mut temp) = current_head{
           self.head = std::mem::replace(& mut temp.next,Option::None);
           return Option::Some(temp.data);
        }
        return Option::None;
     }

     /**
      * Returns the reference of the element at head if there is any.
      */
      //We don't want to return the element. We are retuning the reference to avoid move.

      pub fn peek(& self)->Option<& String>{
        // Note the use of 'if let' with respect to reference. Compiler is really smart. 
        if let Option::Some(cur_head) = & self.head{
            return Option::Some(& cur_head.data);
        } 
        return Option::None
      }

      pub fn iter(& self)->StringStackIter{
        let iter : StringStackIter = StringStackIter { node_ptr: &self.head };
        return iter;
      }

}

impl<'s> Iterator for StringStackIter<'s>{
    type Item = & 's String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Option::Some(temp) = self.node_ptr{
            let ret_val = & temp.data;
            //increment the pointer
            self.node_ptr = & temp.next;
            return Option::Some(ret_val);
        }
        return Option::None;
    }
}

#[test]
pub fn test_string_stack(){
    let mut string_stack : StringStack = StringStack::new();
    string_stack.push(String::from("A")); 
    string_stack.push(String::from("B"));
    string_stack.push(String::from("C"));
    assert_eq!(*(string_stack.peek().unwrap()),String::from("C"));
    let mut i=0;
    let mut iter:StringStackIter = string_stack.iter();
    while let Option::Some(temp_string) = iter.next(){
        if i==0{
            assert_eq!(*temp_string,String::from("C"))
        }else if i==1 {
            assert_eq!(*temp_string,String::from("B"))
        }else if i==2 {
            assert_eq!(*temp_string,String::from("A"))
        }
        i = i+1;
    }
}