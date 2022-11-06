//Asking stack overflow
struct Node{
    data : i32,
    next : * mut Node,
}

struct MyFirstUnsafeQueue{
    head : * mut Node,
    tail : * mut Node,
}
impl MyFirstUnsafeQueue{
    fn new()->Self{
        return MyFirstUnsafeQueue { head: std::ptr::null_mut(), tail: std::ptr::null_mut() };
    }
    /**
     * Adds an element at the rear of the queue
     */
    fn push_1(& mut self, element:i32){
        let mut new_node = Box::into_raw(Box::new(Node{
            data :element,
            next : std::ptr::null_mut(),
        }));
        if self.head.is_null(){
            //https://doc.rust-lang.org/std/primitive.pointer.html#common-ways-to-create-raw-pointers 
            self.head = new_node;
            self.tail = new_node;
        }else{
            unsafe{
                (*self.tail).next = new_node;
                self.tail = new_node;
            }
        }
    }

    fn push_2(& mut self, element:i32){
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
}

#[test]
fn test_push_1(){
    unsafe{
        let mut q:MyFirstUnsafeQueue = MyFirstUnsafeQueue::new();
        q.push_1(1);
        assert_eq!(1, (*q.head).data);
        assert_eq!(1, (*q.tail).data);
        q.push_1(2);
        assert_eq!(1, (*q.head).data);
        assert_eq!(2, (*q.tail).data);
   }
}

//#[test]
fn test_push_2(){
    unsafe{
        let mut q:MyFirstUnsafeQueue = MyFirstUnsafeQueue::new();
        q.push_2(1);
        assert_eq!(1, (*q.head).data);
        assert_eq!(1, (*q.tail).data);
        q.push_2(2);
        assert_eq!(1, (*q.head).data);
        assert_eq!(2, (*q.tail).data);
   }
}
/*

Your test using push_2 exhibits undefined behavior, use-after-free, because new_node is destroyed at the 
end of each call, which will deallocate the contents. The head and tail pointers are dangling when you call .push_2() again.
Your test with push_1 works because Box::into_raw converts the Box into a raw pointer and doesn't deallocate it.

*/