# hello-rust
To learn a language: implement data structures. Rust is no exception. But, we need think in Rust way (probably the best way) to implement data structures. First of all we have to understand the concepts of lifetime, borrow  and ownership of Rust. 
* Note : This is not a rust tutorial

> Hey, Rust's docs is incredibly awesome. https://doc.rust-lang.org/std/

## <b> Rust References </b>
Here I try my best to come up with missing pieces of programming fundamentals unknown to programmer (java programmers like me :) ) In java like programming language we rarely care about memory management. Most memory management related task is taken care by garbage collector. Garbage collector frees the memory when it runs. But, Rust's approach is unique. With ownership borrow concept, Rust able to determine exact points where drop / free can be called. In this journey, I will go through different data structures and algorithms.

### Fundamentals Of Memory Allocation: 
Stack and Heap : Both stack and heaps are parts of memory available to program to use during runtime. If the size of the data is known during compile time, memory is allocated from stack. If size of the data is unknown during compile time it is preferred (In C we can allocate memory from stack using alloc system call !!! )  to allocate memory from heap and starting address of heap memory is returned as pointer which stays in stack memory. It is really  easy to manage memory allocated from stack. When control goes outside the scope of 
a method or function, stack entry associated with it is popped and memory is reclaimed. But, there is a catch, if pointer holding an address from heap; thought the memory allocated to the pointer to store the address of the heap is reclaimed by popping the stack entry, but the memory allocated in heap stays there. OS never follows the pointer to reclaim the memory from heap. 

<br> Memory is allocated from stack for those variable whose size is known at compile time and size is never going to change. For example primitive types like int, long, float, double etc. When these variables are passed to different function as arguments (Note : There is a difference between function parameter and argument :) ) copy of the content is passed which a completely different memory address (Only exception is string literals of Rust. We will discuss ). 

```
fn foo(){
    let i:32=4;
    bar(i);
}
fn bar(j:i32){
  println!("{}",j);
}
```

Each variable going to have their own copy of the data. For example in the above code snippet, when we call bar(), content of 'i' which is a primitive data is copied to 'j'. The address of both i and j are different. When control goes out side the scope, stack entry is popped, memory allocated to these variables are reclaimed. 

<br>But, story is different for memory allocated from heap. We make system call to allocate memory from heap and as a result we get pointer which holds the starting address of the allocated memory from heap. We can't copy the content of heap when we pass the variable as an argument to a method /function call. Because the size of the content is unknown and not fixed. It won't be performant also. Often content of the heap is in GBs and coping them is not a good idea. Therefore, we pass the pointer or  memory address of the heap. The address stored in the pointer variable is copied to the argument of the called function. These pointers stay in stack. Logically a memory address is represented hexadecimal values depending on the size of the addressable memory. 
> Size of the pointer is equal to the size of the address bus from OS prospective. Actual address bus size may be bigger.           

When control goes out side the scope of the function, memory allocated to these pointers to store those hexadecimal values representing memory address of heap are freed. But, memory allocated from heap stays as it is. OS never follow these pointers to reclaim memory from heap. Memory allocated from heap are often the root cause of memory leaks and memory safety issues. Therefore in Java and other java like programming language we have GC. Before Rust it was the responsibility of the programmer to clean the memory. But, trust me it is not at all a easy game.

### Memory leaks:

* Dangling Pointers : A dangling pointer points to the memory location that has already been freed. The storage is no longer allocated. Trying to access it might cause segmentation fault during runtime for pointers holding address of variables created in stack. When control goes out side the scope of a function, local variables (allocated in stack) are freed and if we are returning the address of such variable, we will get a dangling pointer. On similar lines if we are accessing a pointer holding address of heap location which is already reclaimed is also called dangling pointer. Simple C code representing dangling pointer.

 
```
int main(){
   int * ptr;
   ptr = bar();
   printf("%d",*ptr);
   return 0;
}
int * bar(){
  int i=25;
  return &i; // i reclaimed at this point. Therefore &i is a dangling pointer
}
```
* Double Free : A double free leads to undefined behavior. This means that the program can behave completely arbitrarily and all bets are off about what happens. That's certainly a bad thing to have happen! In practice, double-freeing a block of memory will corrupt the state of the memory manager, which might cause existing blocks of memory to get corrupted or for future allocations to fail in bizarre ways (for example, the same memory getting handed out on two different successive calls of malloc).Double frees can happen in all sorts of cases. A fairly common one is when multiple different pointers holding the address of the same memory location and more than one pointer calls free at different point of time.
  
### Reference
> A reference is like a pointer in that it's an address that can be followed to access the data stored in the address; that data is owned by some other variable i.e. ownership is not transferred. Unlike pointer, a reference is guaranteed to point to a valid value of particular type for the life of the reference. Reference can't live longer than variable. Variable could be a smart pointer

> Smart pointer is something which contains a pointer and additional information. For example vector or a string. Smart pointer wraps the actual pointer something similar to class or struct.

* Constant reference to a constant.  
```
  let i:i32 =10; // i is immutable
  let ref:&i32 = &i; // ref is a constant reference to a constant
```

* Constant reference to a mutable variable referred immutably.
```
#[test]
fn foo_05(){
  let mut i:i32 = 5; //mutable variable
  //constant reference to a mutable variable. but variable is referred immutably hence reference can't change the value of i
  let ref_1: & i32 = & i; 
  println!("{}",ref_1);
  //variable is referred immutably hence reference can't change the value of i
  (*ref_1) = 6; //generates compilation error
}
```
* Constant reference to a mutable variable referred mutably. 
 ```
 /**
 * In foo_06() at any point in time, there is always going to single writer. value stored in 'i' can either be changed by
 * ref_1 or 'i' itself. Single writer and multiple reader mutually exclusive. If we borrow immutably, ref_1 won't be valid 
 * anymore.
 */
#[test]
fn foo_06(){
  let mut i:i32 = 5; //mutable variable
  //constant reference to a mutable variable. but variable is referred mutably hence reference can change the value of i
  let ref_1: & mut i32 = & mut i; 
  println!("{}",ref_1);
  (*ref_1) = 6; 
  println!("{}",ref_1);
  println!("{}",i); //println borrows immutably
  // once the immutable borrow happens ref_1 becomes invalid because ref_1 borrows immutably
  //println!("{}",ref_1); 
  //i=10;
  //println!("{}",ref_1);
}
 ``` 

* Mutable reference to a constant.
  ```
  fn foo(){
    let j:i32=10;
    let mut ref_1:& i32 = &j; //reference to a constant. reference is mutable. can refer to another variable
    println!("{}",ref_1);
    let k:i32 =15;
    ref_1 = &k;
    println!("{}",ref_1);
  }
  ```  

* Mutable reference to a mutable value
   ```
  fn foo(){
    let mut j:i32=10;
    let mut ref_1:& mut i32 = & mut j; 
    println!("{}",ref_1);
  }
  ``` 
* Note : We are going to discuss more on references in "Borrowing" section
* Points to remember : We can't have & mut v from an immutable v, where v is a variable. But, we can have & v and & mut v from a mut v. Utility API should always accept & mut v, whether v is mut or not.
  
### Ownership in Rust
Rust approach towards memory safety issue is unique. It comes with a concept of Ownership. The rules of the ownership is quite simple but powerful. It has changed the way I as Java developer think of writing code. These rules are
* Each value in Rust has an owner.
* There can only be one owner at any point in time.
* When owner goes out of scope the value is dropped.
* If the value is copyable i.e. copy type, each owner will get it's own copy. All primitive types are copy type. Stack only values are copyable. 
```
fn foo(){
  let i:i32=6; //copyable value
  let j=i; // value gets copied. memory address of i and j are different
  println("{}",i);
  println("{}",j);
}
```
* String literals are immutable. Because of it's immutable nature, we can have multiple owner of string literal. It doesn't make sense to have multiple copies of immutable data. Copying immutable values is waste of memory.     
```
fn foo_01(){
    let s1:& str = "Hello"; 
    let s2:& str = s1;
    println!("s1 content :{},s1 address{:p}, string literal address:{:p}",s1, & s1, s1);
    println!("s2 content :{},s2 address{:p}, string literal address:{:p}",s2, & s2, s2) ;
}
```
* Note : Refer code to see memory representation in mod hello_reference
* If the a variable mut or not holds a value or address of a value which is immutable, during assignment ownership transfer never happens.
```
/*
  Though 'i' is mutable but value is immutable. So when we perform j=i, value gets copied to the memory 
  location assigned to j. Similarly though s1 is mutable but it holds the address of an immutable element. 
  Therefore, when perform s1=s2, ownership transfer never happens.
*/
#[test]
fn foo_02(){
  let mut i:i32=2;
  let j:i32=i;
  println!("i:{}",i);
  println!("j:{}",j);
  let mut s1:&str = "Hello";
  let s2:&str =s1;
  println!("s1:{}",s1);
  println!("s2:{}",s2);
}

```  
* If the value is mutable, there is always going to be one owner. During assignment operation move or ownership transfer happens making the source uninitialized. Uninitialized variables generate compilation issues if we try to access them. Uninitialized source is completely new for java programmer like me. If someone new in Rust, this particular concept is going to give hard time, particularly writing own linked list based data structure. Single owner rules ensures thread safety. The thumb rule of thread safety : single writer and multiple readers are mutually exclusive.

```
fn foo_03(){
  //String in rust is mutable and memory is allocated from heap. Therefore, can't be copied.
  let s1:String = String::from("Hello Rust"); 
  //When we perform the assignment s2=s1, ownership is transferred.
  let s2:String = s1; 
  println!("s2:{}",s2);
  //After move / transfer of ownership s1 become uninitialized. Hence accessing s1 generates compile time error
  println!("s1:{}",s1);
}
```
> String in Rust is a struct. Struct gets memory from stack unless until programmer want it in heap. String has a Vec internally. Vec gets its memory from heap

* If we perform a deep copy of String by calling clone(), source remain remains valid after the assignment. clone() creates a new memory block in the heap coping entire content from source. Since both memory address are completely different, single owner rules remains valid. But we have to be careful in performing deep copy during runtime. It could impact performance of the application
```
//clone() performs deep copy for String implementation. Copies entire content to a new memory location. 
//Therefore, move doesn't happen for source.
#[test]
fn foo_04(){
  let s1:String = String::from("Hello");
  let s2:String = s1.clone();
  println!("s1:{}",s1);
  println!("s2:{}",s2);
}
```  
### Ownership transfer during method call
* When a method is called and argument is a reference to a mutable element (either a mutable variable or something similar to String. String is a smart pointer) ownership is transferred to the called function / method parameter. (Note : what is the difference between function parameter and argument). Similarly when a method returns a reference (we can only return reference of Heap allocated memory i.e. smart pointer or reference of static element. why ?? we will discuss in our lifetime section) ownership is transferred. String literal is the only exception. TODO code example

* Imagine we need an utility method to calculate length of String. If we pass the String  smart pointer, the caller is going to loose the ownership. But, as good utility API design, utility API shouldn't ask the caller to loose the ownership. One solution could be deep copy. But deep copy can hurt the performance causing denial of service. Other solution could be return the same smart pointer to the caller which is passed as an argument along with the result of the utility function inside a tuple. This is not a readable solution. The solution is reference of the smart pointer. This called borrow in Rust. 

 ### Borrowing
 * Let's build our simple utility to calculate length 
```   
#[test]
fn foo_07(){
  let s:String = String::from("Hello");
  //ownership of s is transferred to the called method. From this point onwards s is an un initialized state
  //We can't use s. But the function my_string_len_cal returns the same String which is passed as an argument
  //and it is assigned to s1. We can use 's1' instead of 's'  
  let (l,s1) = my_string_len_cal(s); 
  assert_eq!(l,5);
  println!("s1:{}",s1);
}
fn my_string_len_cal(s:String)->(usize,String){
  (s.len(), s)
}      
```  
This not a readable code. Can't just return a tuple to implement simple functionality. What's the other approach. Borrow an immutable reference.
``` 
fn bar_1(s_ref:&String)->usize{
  s_ref.len()
}
``` 
* Rust references do not take the ownership. Ownership stays with the variable. Reference simply refer to the value but, doesn't own it.Since the reference doesn't own the variable it points, the value won't be dropped when reference goes out of the scope. References are valid till variable is not dropped (variables are dropped when control goes outside the scope). Referring a value through reference without owning it is called "Borrowing"
  
* What if we want modify something we borrowed ? & mut v (v is a mutable variable i.e. mut v).
  >& mut v (reference which can mutate the value) has one big restriction : If you have a reference which can change the value, you can not have any other references. Remember the golden rule : Single writer and multiple readers are mutually exclusive. The benefit of having this restriction is that Rust can detect data race condition during compile time.
  
  > A data race condition occurs when all of these below occur simultaneously
  <br>1: Two or more pointers access the same data at same time.
  <br>2: At least one of the pointer is being used to write the data.
  <br>3: There no mechanism used to synchronize the access to data. 
  

``` 
fn bar(){
  let mut s:String = String::from("Hello");
  let r1 = & mut s;
  let r2 = & mut s;
  println!("r1:{},r2:{}",r1,r2);
}
```
Above code will not compile. We can not borrow 's' as mutable more than once and keep using them all. We can use only one. Refer below code. Though we have r1 and r2, we just using r1.
```
#[test]
fn bar(){
  let mut s=String::from("Hello");
  let r1 = &s;
  let r2 = &s;
  println!("{}",r1)
}
//Also refer hello_ref::foo_10()
```
* We can have multiple immutable references. No issues.
* We can not have a mutable reference while we have an immutable reference of the same variable. & v and & mut v is mutually exclusive where v is a mutable variable.
* Dangling references can't be possible. We can not borrow a reference if the variable is dropped. below code will not compile. We are returning &s. When control goes outside of bar(), s is dropped and &s become invalid.
 ```
 fn foo(){
  ler ref_1=bar();
 }
 fn bar()-> &String{
  let s = String::from("Hello");
  return &s; 
 }
 ``` 
 ### Lifetime
 * Lifetime is completely a new concept for me. Lifetimes are associated with reference. It tells the Rust compiler what is the maximum life of a reference.
 * Rule is simple lifetime of reference <= scope of the variable. Because of this principle below code will not compile
```
 fn foo(){
  ler ref_1=bar();
 }
 fn bar()->& i32{
  let i:i32 = 10;
  return &i; 
 }
 ```
 Compiler trying to know the lifetime of the reference. As we are returning reference of a local variable,compiler is complaining. When control goes outside the scope of bar(), local variables are dropped causing the reference to dangle. Lifetime protects us from creating dangling references

 * What is static lifetime ? Local variable stays as long as control is inside. static variable can stay outside. 
 ```
 fn foo(){
  ler ref_1=bar();
 }
 static NUM:i32=13;
 fn bar()-> & 'static i32{
  return & NUM; 
 }
 ```    
* Lifetime are what the Rust compiler uses to keep track of how long references are valid. Lifetime helps the borrow checker to ensure that we never going to have invalid references.

* What is borrow checker ? When we create a reference of a variable, we say we borrowed the reference. Borrow checker mechanism keeps track of the reference to determine exactly when drop() should be called to avoid memory safety bugs and memory leaks. Suppose we return reference of a local variable or reference of smart pointer or reference of string literal (recall when we declare string literal it is always a reference) implicit lifetime associated with these reference ends inside function scope. Therefore, we can't access the reference.    

* Every local variable is defined with an implicit lifetime. We can't override the lifetime of reference of local variable. It is a design decision which keeps things clean and simple. Otherwise, Rust programmer would have to chase these lifetime. Imagine a situation we have 10 layers of call stack and each returning different lifetime to the caller.

* Suppose we have a vector and we have some util to populate the vector with String reference as we don't want to own the value. We know variable are dropped as soon as control goes outside the scope of the function. Reference can't live when variable is dropped. But, can we can ask the Rust compiler to keep the reference with for cretin time period by defining lifetime. No !!! A Big no. (or else our lif would be chasing the rabbit hole of reference) Hold on one more point can we pass the vector variable to the called function. We can but we will loos the ownership. Too many stuffs !!! isn't it ?? Below code will not compile.

```
/**
 * 
 */
#[test]
fn foo_08(){
  //Vector accepting list of String reference
  let mut l: Vec<& String> = Vec::new();
  bar_08(&mut l);

}
fn bar_08 <'a> (l:& 'a mut Vec<& 'a String>){
   let s1:String = String::from("Hello");
   let s1_ref : & 'a String= 'a & s1; 
   l.push(s1_ref);
}
error[E0597]: `s1` does not live long enough
   --> src/hello_reference/hello_ref.rs:142:30
    |
140 | fn bar_08 <'a> (l:& 'a mut Vec<& 'a String>){
    |            -- lifetime `'a` defined here
141 |    let s1:String = String::from("Hello");
142 |    let s1_ref : & 'a String= &  s1; 
    |                 -----------  ^^^^^ borrowed value does not live long enough
    |                 |
    |                 type annotation requires that `s1` is borrowed for `'a`
```
* Is there any way to return reference ? Wrapping it inside a variable. Variable could be a struct or a smart pointer like vector ? No Big No !! No way. Again reiterating things dangling reference is not possible in Rust. We can't trick the compiler. When variable is dead reference can't live. Reference can't outlive the variable. If we think deeply, there is absolutely no use case of such type. If we wish to return a value, return the variable along with ownership.
  
* While defining reference of a variable we cannot enforce the reference to have a particular lifetime. Rust doesn't allow references declared with lifetime. Otherwise, that would be a nightmare for programmer to chase those lifetime to figure out dangling references.

* The rule is pretty simple 
>Always pass reference as argument and return variable from function.But you can always return a reference from a method if data the referred by the reference belongs to struct. The single factor always need to be considered while returning reference "Rust never allows dangling references." We can keep a reference of a variable as an attribute in struct. The lifetime of such reference must be grater than equal to the lifetime of struct variable. The lifetime of reference strictly follow the stack. If lifetime of reference 'r1' pushed to the stack before the struct variable 's' 's' can access 'r1'. The stack principle is not just for lifetime or refer 

* When we define function, method, struct or enum and we want to use reference we can mention lifetime of the reference. In case of struct or enum it tells the compiler what is the lifetime of the reference used inside the struct or enum . 
```
struct Foo<'s>{
  data : & 's String,
}  
```
Here we are saying lifetime of variable of type Foo is less than equal to lifetime of data which is a reference to a String.

* Rc and Box : Rc and Box are smart pointers to allocate memory from heap. Rc stand for reference counted. When we use Rc, Rc keeps track of pointer holding the address of the memory location allocated from the heap for the same Rc pointer. Box is equivalent to new operator in Java. More detailed documentation is there in Rust docs. When I start using these two smart pointers (and their siblings; they have siblings too) I must admit I got confused because in Java, we do <br>
 ```
 public class User{
  String name;
 }
 User user = new User();
 user.name = "ab";
 System.out.println(user.name);
 ``` 
 the new operator return type is the Object itself for which it is invoked. Reading and writing is done jst through the reference. Now let's see its equivalent in Rust <br>
 ```
pub struct User{
  name : String
}

let mut user : Box<User> = Box::new(User{name : String::from("cd")});
user.name = "ab"
println!("{}",user.name);
```
For me in Rust the type of user variable is bit unreadable initially. Perhaps due to my experience with Java. treat Box, Rc and its siblings as new operator in java. These smart pointer gives direct access to the type they wrap.

## A closer look on Box, Rc and RefCell
Box is a pointer to heap. In Rust we don't have null, therefore the Box pointer is kept inside Option. Option enum keeps the pointer. The type of next is not readable when compare it to C or Java. Box is same as new in Java. new has return value of class it self but, Box has type Box<T> but using the variable of Box<T> we can access attributes of T 
```    
struct Node{
    data : i32,
    next : Option<Box<Node>>,
}
```
In linked list tail and previous node of tail points to the same memory address. Box won't allow multiple reference holding same memory location. When the size of the queue will be 1, both head and tail point to the same memory location. During push tails will try to append new node and during pop head will try to remove node from head. To design the queue our only option is Option<Rc<RefCell<Node>>>. Another problem with blow structure is interior mutability. When we add a new node at the rear end we need to update the next pointer of tail. But, incase stack next pointer of new node holds the address of head and address of head is always known value while setting value new node.

```
struct MyUnsafeQueue{
    head : Option<Box<Node>>,  
    tail : Option<Box<Node>> 
}
```

Note  : Option<Box<Node>> will work if an only if addition and deletion happens either at front or rear. And the data structure
can only keep that pointer. For example stack. In case of stack, push and pop happens at head and we just need to keep the pointer
of head
```
Stack{
    head : Option<Box<Node>>
}
```
## Destructor (TODO)

## Implemented data structures:
I am going to implement following data structures and algorithms. I won't be explaining those. Code has explanations why it is done that way. I have kept the explanation as simple as possible. Linked list based data structure is a good starting point.

> Linked lists were a great innovation for simple computer architectures similar to the 1970-era PDP-11, but they defeat most of the hardware acceleration features of modern architectures with their parallel and pipelined execution resources. Multi-level memory caches, single- and multi-issue instruction pipelines, look-aside address translation and branch prediction caches, parallel arithmetic and logical execution units, etc. all end up stalling or being massively under-utilized when chasing linked lists. Think about false sharing while using linked list

* Follow the below sequence
* Stack : Linked list based stack my_stack
* my_persistent_stack
* my_linked_list
* doubly linked list
* single linked list with mutable iterator

## Raw pointers and Unsafe Rust 
Unsafe Rust exists because, by nature, static analysis is conservative. When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to reject some valid programs than to accept some invalid programs. Although the code might be okay, if the Rust compiler doesn’t have enough information to be confident, it will reject the code. In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.” Be warned, however, that you use unsafe Rust at your own risk: if you use unsafe code incorrectly, problems can occur due to memory unsafety, such as null pointer dereferencing.Rust needs to allow  to do low-level systems programming, such as directly interacting with the operating system or even writing  own operating system. 

* Raw pointers can either be mutable or immutable which is similar to C pointers and pointers to constant respectively  
```
let mut num = 5;
let r1 = &num as *const i32; 
let r2 = &mut num as *mut i32;
```
'r1' declared as pointer to constant. since num is mutable we can either have a mutable pointer or pointer to a constant. To define raw pointers unsafe block is not required 
> We can't have a mutable raw pointer '&mut num as *mut' from a immutable variable. Exactly same as reference. We can't have a '& mut' from immutable variable.

* We can create a raw pointer from any arbitrary memory location
```
fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
}
```
* Unsafe block allows dereference a raw pointer. Creating a pointer does no harm; it’s only when we try to access the value that it points at that we might end up dealing with an invalid value. 
```
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
/*
Below code will not compile.
*/
fn main() {
    let mut num = 5;
    let r1 = & num;
    let r2 = & mut num;
    
    println!("{}",r1);
    println!("{}",r2);
}

```
> Note we have created *const i32 and *mut i32 raw pointers that both pointed to the same memory location, where num is stored. If we instead tried to create an immutable and a mutable reference to num, the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references. With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race. Be careful!   

* We can have multiple mutable raw pointer. This is not possible with reference
 ```
 fn main() {
    let mut num = 5;
    let r1 = &mut num as *mut i32;
    let r2 = &mut num as *mut i32;
    unsafe{
    println!("{}",*r1);
    println!("{}",*r2);
    }
}
``` 
Refer hello_unsafe module in src to find more details. I have created a single linked list using raw pointers. 

* Places where unsafe code is the only way
  <br>1: Dereference a raw pointer (already discussed)
  <br>2: Call an unsafe function or method
  <br>3: Access or modify a mutable static variable
  <br>4: Implement an unsafe trait
  <br>5: Access fields of unions (to access C union)

## Reference
* https://doc.rust-lang.org/nightly/nomicon/intro.html 
* https://rust-unofficial.github.io/too-many-lists/index.html