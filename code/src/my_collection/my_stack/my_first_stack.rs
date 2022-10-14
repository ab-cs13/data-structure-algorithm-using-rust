
//In stack implementation we are going to accept immutable reference of any type as data. By this approach we are not going to own the
//value. Since stack accepting reference, it needs lifetime. The lifetime of reference (the data) must be >= lifetime of the stack.
//Stack always going to have a head. All push and pop operations are performed at head only.

struct Node<'a, T >{
    data: & 'a T,
    next : NextPtr<'a,T>,
}
//Rust doesn't have null. We need a marker to indicate the last element of stack. Rust has avoided the 
//billion dollar mistake.  We can use the in-built Option enum (recommended).As we are learning Rust, lets create our own enum. 

//NextPtr could be EMPTY or hold the address of the next node. Therefore, the definition of next pointer is  
enum NextPtr<'b, T>{
    EMPTY, //NextPtr of stack could be EMPTY which represents NULL in other programming language.
    //why the NONEMPTY holds a box type ? 
    NONEMPTY(Box<Node<'b, T>>)
}
//head of the stack could be EMPTY (at the beginning) and NONEMPTY when elements are pushed in the stack.
//Lifetime parameter is there as we are dealing with references.
pub struct MyFirstStack<'s, T>{
    head : NextPtr<'s, T>,
}
impl <'s,T>MyFirstStack<'s,T>{
    //creates a stack with EMPTY value
    pub fn new()->Self{
        let stack = MyFirstStack{
            head:NextPtr::EMPTY,    
        };
        return stack;
    }
    // Ok, what we have to do ? Let's first write it down
    // 1: head is holding or pointing or in Rust terminology owning starting address of the stack. Let's assume starting address is 1000AB
    // 2: Create a new node
    // 3: Till this point approach is same in all programming language. In other languages 
    //    i) new_node.next = head
    //    ii) head = new_node
    // But, in Rust can't do that if we do new_node.next = head, ownership of the memory location '1000AB' will be transferred to
    // new_node.next and head becomes unutilized in the control flow. This is absolutely not permitted. Here we take a different approach 
    // we are going to use std::mem::replace function, to set some temporary value in head and get the current value stored in head.
    // The returned value of replace function call will be assigned to new_node.next.
    pub fn push(& mut self, input:& 's T){
       let temp=std::mem::replace(& mut self.head, NextPtr::EMPTY); 
       // In Java class (equivalent to struct) can't get memory from stack. Only from heap we can allocate memory. This is because Java
       // GC, manages the heap memory.
       // In C/C++ we can get memory for struct from heap using malloc and from stack using alloc. 
       // But, in Rust we can allocate memory of struct from stack. Why we need wrap in Box i.e. allocate it again from heap ? 
       // When control goes out of the scope, stack gets cleared but we need the data even control goes outside the
       // scope of push method. 
       let new_node = Node{
          data:input,  
          next:temp
       };
       self.head = NextPtr::NONEMPTY(Box::new(new_node));
    }
    //pop method returns the reference of the element stored at head and deletes it.
    pub fn pop(& mut self)->Option<& 's T>{
     //we can't just do 
     // 1: temp=head;
     // 2: head = head.next;
     // If we perform step 1, head will be uninitialized and this is not allowed. Therefore we have to use std::mem::replace
     // to put some temp element in head
     let temp_node = std::mem::replace(& mut self.head, NextPtr::EMPTY);
     match temp_node{
        NextPtr::EMPTY => Option::None,
        NextPtr::NONEMPTY(next_element)=>{
            self.head = next_element.next;
            Option::Some(next_element.data)    
        }
     }
     
    }
}
#[test]
fn test_my_first_stack(){

}
