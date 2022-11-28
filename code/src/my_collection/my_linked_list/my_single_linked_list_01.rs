use std::{rc::Rc, cell::RefCell};

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

    fn mut_iterator(& 'l mut self)->MutIterator<'l,T>{
        return MutIterator { cur_ptr: self.head.clone(), prev_ptr: Option::None, ll: self }
    }

}

struct MutIterator<'i,T>{
    cur_ptr : Option<Rc<RefCell<Node<'i,T>>>>,
    prev_ptr : Option<Rc<RefCell<Node<'i,T>>>>,
    ll : & 'i mut MyFirstSinglyLL<'i,T>
}


impl<'i,T> Iterator for MutIterator<'i,T>{
    type Item = & 'i T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_ptr.is_none(){
            return Option::None;
        }else{
            let ret_val = self.cur_ptr.as_ref().unwrap().borrow().data;
            self.prev_ptr = self.cur_ptr.clone();
            self.cur_ptr = self.prev_ptr.as_ref().unwrap().borrow().next.clone();
            return Option::Some(ret_val);
        }
    }

}
impl <'i,T> MutIterator<'i,T>{
    
    /**
     * put the element in the current position and push the current element to the next position
     */
    fn put(& mut self, element:&'i T){
        let new_node  = Rc::new(RefCell::new(Node{
            data : element,
            next : Option::None,
        }));
        if self.prev_ptr.is_none(){
            //next ptr is at head.
            if self.cur_ptr.is_none(){
                //linked list is empty
                self.ll.head = Option::Some(new_node.clone());
                self.ll.tail = Option::Some(new_node.clone());
                self.cur_ptr = Option::Some(new_node.clone());
            }else{
                //linked list has some element. We don't need to update the tail
                new_node.borrow_mut().next = self.ll.head.clone();
                self.ll.head = Option::Some(new_node.clone());
                self.cur_ptr = Option::Some(new_node.clone());
            }
        }else{
            if self.cur_ptr.as_ref().is_none(){ //Adding at tail
                //self.cur_ptr.as_ref().unwrap().borrow_mut().next = Option::Some(new_node.clone());
                self.ll.tail = Option::Some(new_node.clone());
                self.prev_ptr.as_ref().unwrap().borrow_mut().next = Option::Some(new_node.clone());
                self.cur_ptr = Option::Some(new_node.clone());
            }else{
                //At any position other than head and tail
                new_node.borrow_mut().next = self.cur_ptr.clone();
                self.prev_ptr.as_ref().unwrap().borrow_mut().next = Option::Some(new_node.clone()); 
            }
        }
    }
    /**
     * Deletes the current element. 
     */
    fn delete(& mut self)->Option<& 'i T>{
    
        if self.cur_ptr.is_none(){
            //if linked list is empty
           return Option::None;
        }else {
            let ret_val = self.cur_ptr.as_ref().unwrap().borrow().data;
            if self.prev_ptr.is_none(){
                //head node
                if self.cur_ptr.as_ref().unwrap().borrow().next.is_none(){
                    //Only one node
                    self.cur_ptr = Option::None;
                    self.ll.head = Option::None;
                    self.ll.tail = Option::None;
                }else{
                    let new_head = self.cur_ptr.as_ref().unwrap().borrow().next.clone();
                    self.cur_ptr = new_head.clone();
                    self.ll.head = new_head.clone();
                }
               
            }else{
                self.prev_ptr.as_ref().unwrap().borrow_mut().next = self.cur_ptr.as_ref().unwrap().borrow().next.clone();
                self.cur_ptr = self.prev_ptr.as_ref().unwrap().borrow().next.clone();

            }
            return Option::Some(ret_val);
        }
    }
    
    fn refresh(& mut self){
        self.cur_ptr = self.ll.head.clone();
        self.prev_ptr = Option::None;
    }
}

//TODO implement drop trait

#[test]
fn test_append_ll(){
   let s1: &String = & String::from("A");
   let s2: &String = & String::from("B");
   let s3: &String = & String::from("C");
   let mut ll : MyFirstSinglyLL<String> = MyFirstSinglyLL ::new();
   ll.append( s1);
   ll.append( s2);
   ll.append( s3);

   let mut it:MutIterator<String> = ll.mut_iterator();
   let e1 = it.next().unwrap();
   assert_eq!(s1,e1);
   assert_eq!(s2,it.next().unwrap());
   assert_eq!(s3,it.next().unwrap());
}

#[test]
fn test_put_at_start_ll(){
    let s1: &String = & String::from("A");
    let s2: &String = & String::from("B");
    let s3: &String = & String::from("C");
    let mut ll : MyFirstSinglyLL<String> = MyFirstSinglyLL ::new();
    ll.append( s2);
    ll.append( s3);
 
    let mut it:MutIterator<String> = ll.mut_iterator();
    it.put(s1);
    assert_eq!(s1,it.next().unwrap());
    assert_eq!(s2,it.next().unwrap());
    assert_eq!(s3,it.next().unwrap());
 }
 #[test]
 fn test_put_at_end_ll(){
    let s1: &String = & String::from("A");
    let s2: &String = & String::from("B");
    let s3: &String = & String::from("C");
    let s4: &String = & String::from("D");
    let mut ll : MyFirstSinglyLL<String> = MyFirstSinglyLL ::new();
    ll.append( s1);
    ll.append( s2);
 
    let mut it:MutIterator<String> = ll.mut_iterator();
    assert_eq!(s1,it.next().unwrap());
    assert_eq!(s2,it.next().unwrap());
    it.put(s3);
    assert_eq!(s3,it.next().unwrap());
    it.put(s4);
    assert_eq!(s4, it.next().unwrap());
    //refresh the iterator to iterate from the 0th index.
    it.refresh();
    assert_eq!(s1,it.next().unwrap());
    assert_eq!(s2,it.next().unwrap());
    assert_eq!(s3,it.next().unwrap());
    assert_eq!(s4, it.next().unwrap());
    
 }
//TODO test del using iterator


