
//In stack implementation we are going to accept immutable reference of any type as data. By this approach we are not going to own the
//value. Since stack accepting reference, it needs lifetime. The lifetime of reference (the data) must be >= lifetime of the stack.
//Stack always going to have a head. All push and pop operations are performed at head only.

struct Node<'a, T >{
    data: & 'a T,
    next : NextPtr<'a,T>,
}
//Rust doesn't have null. Rust has avoided the billion dollar mistake. We need a marker to indicate the last element of stack. 
//We can use the in-built Option enum (recommended).As we are learning Rust, lets create our own enum. 

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
    /* 
    Push an element to the head of the stack. 
    Ok, what we have to do ? Let's first write it down
     1: head is holding or pointing or in Rust terminology owning starting address of the stack. Let's assume starting address is 1000AB
     2: Create a new node
     3: Till this point approach is same in all programming language. In other languages 
        i) new_node.next = head
        ii) head = new_node
     But, in Rust can't do that if we do new_node.next = head, ownership of the memory location '1000AB' will be transferred to
     new_node.next and head becomes unutilized in the control flow. This is absolutely not permitted. Here we take a different approach 
     we are going to use std::mem::replace function, to set some temporary value in head and get the current value stored in head.
     The returned value of replace function call will be assigned to new_node.next.
    */
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


 /*
      Do we need to implement drop trait ? First few points about drop trait. Drop trait looks something like this
       pub trait Drop {
           fn drop(&mut self);
       }
      
      When a variable (primitive types, struct, enum and smart pointers) goes outside the scope, Rust will run a "destructor"
      (a term coined from C++). This is one of the most common situation. There are other situations too.
      The destructor consists of two components

        1:A call to Drop::drop for that value, if this special Drop trait is implemented for its type.
        2:The automatically generated “drop glue” which recursively calls the destructors of all the fields of this value.
     
      Note : drop calls are recursive. In our case these recursive call could blow up the stack. We have allocated memory from heap and 
      allocated heap memories could be larger than stack. One solution is tail recursion. Because tail recursion can be can be coveted to
      a loop.
    
      -----------------------
      Tail call optimization:
      -----------------------
      Let's have a simple function to calculate sum of first N no of integers. We know we can do that by simple N(N+1)/2. Let's
      do it recursively
      
      fn sum(n:i32)->i32{
        if n==0 {
            return 0;
        }else {
            return n + sum(n-1);
        }
      }
      
      Let n=5; Now how it looks like in the stack
      
       ______________________ 
      |        5             |   5 + sum(4)
      |______________________|
      |        4             |   5 + (4 + sum(3))
      |______________________|
      |        3             |   5 + (4 + (3 + sum(2)))
      |______________________|
      |        2             |   5 + (4 + (3 + (2 + sum(1))))
      |______________________|
      |        1             |   5 + (4 + (3 + (2 + (1 + sum(0)))))
      |______________________|
      |        0             |   5 + (4 + (3 + (2 + (1 + 0))))
      |______________________|

      
    sum(5)
    5 + sum(4)
    5 + (4 + sum(3))
    5 + (4 + (3 + sum(2)))
    5 + (4 + (3 + (2 + sum(1))))
    5 + (4 + (3 + (2 + (1 + sum(0)))))
    5 + (4 + (3 + (2 + (1 + 0))))
    5 + (4 + (3 + (2 + 1)))
    5 + (4 + (3 + 3))
    5 + (4 + 6)
    5 + 10
    15

    So the stack size is equal to the value of the input. It could blow the system. (same with our drop call for our stack)

    fn tail_rec_sum(n:i32, running_total: mut i32) {
        if n === 0 {
            return running_total;
        } else {
        return tail_rec_sum(n - 1, running_total + n);
        }
    }
    the accumulator of result "running_total" does the trick here. We don't need to pop the stack to get the final output.
    Compiler can perform the optimization to reduce the space complexity. 
    Compiler can produce below pseudo code
    
    tail_rec_sum(n:i32, running_total: mut i32){ //signature remains same
       while n!=0 { //if replaced by while with opposite condition
         running_total = running_total + n //2nd part of of recursive call
         n = n-1; // loop invariant of recursion 
       }
      return running_total; //return for terminal operation moved to the end.
    }
    This is only for understanding purpose. We need to see the object file to find out exact code emitted by the compiler.
    We will discuss it latter.(ref https://eklitzke.org/how-tail-call-optimization-works )
    
    Now lets get back to our problem. We need to fid whether drop code generated by compiler for our stack implementation is tail
    recursive or not. If not, is there anyway we can help compiler. 
      
     impl Drop for MyFirstStack {
        fn drop(&mut self) {
        // NOTE: you can't actually explicitly call `drop` in real Rust code;
        // we're pretending to be the compiler!
        self.head.drop(); // tail recursive - good!
        }
    }

    impl Drop for NextPtr {
        fn drop(&mut self) {
            match *self {
                NextPtr::EMPTY => {} // Done!
                NextPtr::NONEMPTY(ref mut boxed_node) => {
                    boxed_node.drop(); // tail recursive - good!
                }
            }
        }
    }

    impl Drop for Box<Node> {
        fn drop(&mut self) {
            self.ptr.drop(); // uh oh, not tail recursive!
            deallocate(self.ptr); //free the pointer and memory location of heap it points
        
        }
    }
    
   impl Drop for Node {
        fn drop(&mut self) {
            self.data.drop();
            self.next.drop();
        }
    }
    
    drop code for Box isn't tail recursive. we can't call drop after deallocate. 
    If we do that we are going to lose next node memory location. 

    Let's implement drop for MyFirstStack to help compiler to make all drop recursive call to tail recursive.

    When drop is called, we are going to replace all node to EMPTY is our while loop. rest drop calls are tail recursive
    */ 
    impl  <'s,T> Drop for MyFirstStack<'s, T>{
        fn drop(&mut self) {
            let mut current : NextPtr<T> = std::mem::replace(& mut self.head,NextPtr::EMPTY);
            while let NextPtr::NONEMPTY(mut node) = current{
                current = std::mem::replace(& mut node.next, NextPtr::EMPTY); 
            }
        }
    }    
#[test]
fn test_my_first_stack(){

}
