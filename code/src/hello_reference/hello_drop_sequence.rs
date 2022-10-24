/*
 Interesting behavior with Rust lifetime and Drop trait 
*/
/*
struct Foo<'a>{
    data : & 'a String,
}
impl<'a> Drop for Foo<'a>{
    fn drop(&mut self) {
       
    }
}

#[test]
fn test_foo(){
   let s1:String = String::from("A"); 
   let mut foo:Foo = Foo { data: & s1 };
   let s2:String = String::from("B");
   foo.data = & s2; // compilation error : s2 does not live long enough.
}
If I implement Drop trait for Foo struct
foo.data = & s2; 
starts giving compilation error : s2 does not live long enough. 
The error makes sense to me. But, why am I not getting the same error if I don't implement Drop trait for Foo


In Rust, local variables (from the same scope) are dropped in reverse order of the order they're defined.
We declare three local variables here: s1, foo, and s2, in that order. Rust would like to drop them in reverse order: s2 first, 
then foo, then s1. However, there's a problem with the lifetimes. Once we drop s2, then foo.data is uninitialized, 
i.e. it points to garbage memory.

Now, why does this work without Drop? Rust has a concept called partial moves. If you have a structure that has multiple fields, 
Rust will allow you to move out of some fields without invalidating the whole struct. In principle, if I have a
struct Person {
  name: String,
  age: i32,
  occupation: String,
}

and I do let name = my_person.name; (where my_person: Person), then I've moved a value out of a Person. 
Thus, my_person.name is invalid, and in the absence of partial moves, Rust should consider my_person completely invalid.
However, we know that my_person.age and my_person.occupation are still valid, so Rust will let age and occupation remain in place.
It remembers that name is moved (and hence garbage that should not be dropped), while age and occupation are still valid.

In your example, the same thing is happening. Rust wants to drop s2, but Foo still holds a reference to it. 
Rust considers that fine: We'll drop s2 and simply say that Foo has been partially moved: Its data field is no longer valid. 
Then when we go to drop foo next, we don't need to drop the reference, merely the outermost Foo layer itself.

Without a Drop instance, this is fine, and Rust will allow it. However, if impl<'a> Drop for Foo<'a> is in scope, 
then partial moves are completely disabled for Foo. Rust sees that you're implementing some custom Drop behavior, 
and now it won't allow a partially-initialized object to exist, since we would have to drop a partially-initialized object, 
and Rust can't predict what your custom Drop code is going to do or what assumptions it's going to make.

So with a Drop implementation, Rust still wants to drop s2 first, but it can't partially move the reference out of foo, 
since that would leave foo in a partially-initialized state, which isn't allowed.
*/