use std::{rc::Rc, cell::RefCell,};

struct Node<'a, T>{
    data : & 'a T,
    next : Option<Rc<RefCell<Node<'a, T>>>>,
}

struct MyFirstSinglyLL<'l, T>{
    head : Option<Rc<RefCell<Node<'l,T>>>>,
    tail : Option<Rc<RefCell<Node<'l,T>>>>, 
}

impl <'l,T> MyFirstSinglyLL<'l,T>{
    fn new()->Self{
        return MyFirstSinglyLL { head: Option::None, tail: Option::None }
    }
    /**
     * Adds an element at the rear end of the list.
     */
    fn append(& mut self, element : & 'l T){
        let new_node = Rc::new(RefCell::new(Node{
            data : element,
            next : Option::None,
        }));
        if self.head.is_none(){
            self.head = Option::Some(new_node);
            self.tail = self.head.clone();
        }else{
            self.tail.as_ref().unwrap().borrow_mut().next = Option::Some(new_node.clone());
            self.tail = Option::Some(new_node.clone());
        }
    }

    fn mut_iterator(& 'l mut self)->MutItertor<'l,T>{
        return MutItertor { cur_ptr: self.head.clone(), prev_ptr: Option::None, the_ll: self }
    }

}

struct MutItertor<'i,T>{
    cur_ptr : Option<Rc<RefCell<Node<'i,T>>>>,
    prev_ptr : Option<Rc<RefCell<Node<'i,T>>>>,
    the_ll : & 'i mut MyFirstSinglyLL<'i,T>
}