/*
In this implementation we are going to use raw pointers. Code will be more or less like C
 
Box is a pointer to heap. In Rust we don't have null, therefore the Box pointer is kept inside Option
Option enum keeps the pointer. The type of next is not readable when compare it to C or Java. Box is 
same as new in Java. new has return value of class it self but, Box has type Box<T> but using the variable
of Box<T> we can access attributes of T 
    
struct Node{
    data : i32,
    next : Option<Box<Node>>,
}

In linked list tail and previous node of tail points to the same memory address. Box won't allow multiple 
reference holding same memory location. When the size of the queue will be 1, both head and tail point
to the same memory location. During push tails will try to append new node and during pop head will try
to remove node from head. To design the queue our only option is Option<Rc<RefCell<Node>>>. 
Another problem with blow structure is interior mutability. When we add a new node at the rear end we need to update
the next pointer of tail. But, incase stack next pointer of new node holds the address of head and address of head is always
known value while setting value new node.

struct MyUnsafeQueue{
    head : Option<Box<Node>>,  
    tail : Option<Box<Node>> 
}

Note  : Option<Box<Node>> will work if an only if addition and deletion happens either at front or rear. And the data structure
can only keep that pointer. For example stack. In case of stack, push and pop happens at head and we just need to keep the pointer
of head
Stack{
    head : Option<Box<Node>>
}

*/  

struct Node{
    data : i32,
    //Note : we don't need unsafe block to declare raw pointer. It is only required during dereferencing
    next : * mut Node,
}

struct MyFirstUnsafeQueue{
    head : * mut Node,
    tail : * mut Node,
    //Well, two mutable raw pointers are allowed but this won't be 
}
impl MyFirstUnsafeQueue{
    fn new()->Self{
        return MyFirstUnsafeQueue { head: std::ptr::null_mut(), tail: std::ptr::null_mut() };
    }
    /**
     * Adds an element at the rear of the queue
     */
    fn push(& mut self, element:i32){
        let mut new_node = Box::new(Node{
            data :element,
            next : std::ptr::null_mut(),
        });
        if self.head.is_null(){
            //https://doc.rust-lang.org/std/primitive.pointer.html#common-ways-to-create-raw-pointers 
            self.head = & mut * new_node;
            self.tail = & mut * new_node;
        }else{
            unsafe{
                (*self.tail).next = & mut *new_node;
                self.tail = & mut * new_node;
            }
        }
    }
    /**
     * Removes the element from head and returns the data
     */
    fn pop(& mut self)->Option<i32>{
        if self.head.is_null(){
            return Option::None;
        }else{
            unsafe{
                let ret_val : i32 = (*self.head).data;
                self.head = (*self.head).next;
                if self.head.is_null(){
                    self.tail = std::ptr::null_mut();
                }

                return Option::Some(ret_val);
            }
        }
    }

    fn iter(& self)->Iter{
        return Iter { ptr: self.head };
    }
}

struct Iter{
    ptr : * const Node,
}

impl Iterator for Iter{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.ptr.is_null(){
            unsafe{
            let ret_val : i32 = (*self.ptr).data;
            self.ptr = (*self.ptr).next;    
            return Option::Some(ret_val);
            }
        }else{
            return Option::None;
        }
    }
}
/*
mid-level intermediate representation interpreter (MIRI)

An experimental interpreter for Rust's mid-level intermediate representation (MIR). It can run binaries and test suites of cargo projects and detect certain classes of undefined behavior, for example:

    1: Out-of-bounds memory accesses and use-after-free
    2: Invalid use of uninitialized data
    3: Violation of intrinsic preconditions (an unreachable_unchecked being reached, calling copy_nonoverlapping with overlapping ranges, ...)
    4: Not sufficiently aligned memory accesses and references
    5: Violation of some basic type invariants (a bool that is not 0 or 1, for example, or an invalid enum discriminant)
    6: Experimental: Violations of the Stacked Borrows rules governing aliasing for reference types
    7: Experimental: Data races (but no weak memory effects)

    On top of that, Miri will also tell you about memory leaks: when there is memory still allocated at the end of the execution, and that memory is not reachable from a global static, Miri will raise an error.

    ...

    However, be aware that Miri will not catch all cases of undefined behavior in your program, and cannot run all programs

*/
#[test]
fn test_1(){
   let mut q:MyFirstUnsafeQueue = MyFirstUnsafeQueue::new();
   q.push(1);
   q.push(2);
   q.push(3);

   let mut iter = q.iter();
   
   let mut i:i32 =1;
   while let Option::Some(x)=iter.next(){
    assert_eq!(i,x);
    i=i+1;
   }

}