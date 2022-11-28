struct Node{
    data : i32,
    next : Option<Box<Node>>,
}

struct MyQueue<'q>{
    head : Option<Box<Node>>,
    tail : Option<& 'q Box<Node>> 
}
impl <'q>MyQueue<'q>{
    /**
     * Creates a new queue. Naming the method name new is just a convention. It could be anything.
     */
    fn new()->Self{
        return MyQueue { head: Option::None, tail: Option::None }
    }
    fn append(& mut self, new_element : i32){
        let new_node = Box::new(Node{
            data : new_element,
            next : Option::None,
        });
        if self.head.is_none() {
            self.head = Option::Some(new_node);
            //self.tail = Option::Some(& 'q new_node);
        }
    }
}