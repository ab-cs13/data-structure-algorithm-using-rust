# hello-rust
To learn a language: implement data structures. Rust is no exception. But, we need think in Rust way (probably the best way) to implement data structures. First of all we have to understand the concepts of lifetime, borrow  and ownership of Rust. 
* Note : This is not a rust tutorial
## Path
We will start with concepts pointers in C to understand references of Rust. Then we will visit to other fundamental concepts.

## <b> Rust References </b>
Here I try my best to come up with missing pieces of programming fundamentals unknown to programmer (java programmers :) ) In java like programming language we rarely care about memory management. 

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
* Double Free

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

## Implemented data structures:
I am going to implement following data structures and algorithms. I won't be explaining those. Code has explanations why it is done that way. I have kept the explanation as simple as possible. Linked list based data structure is a good starting point.

> Linked lists were a great innovation for simple computer architectures similar to the 1970-era PDP-11, but they defeat most of the hardware acceleration features of modern architectures with their parallel and pipelined execution resources. Multi-level memory caches, single- and multi-issue instruction pipelines, look-aside address translation and branch prediction caches, parallel arithmetic and logical execution units, etc. all end up stalling or being massively under-utilized when chasing linked lists. Think about false sharing while using linked list

* Follow the below sequence
* Stack : Linked list based stack my_stack/my_first_stack.rs

