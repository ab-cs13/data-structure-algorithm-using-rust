# hello-rust
To learn a language: implement data structures. Rust is no exception. But, we need think in Rust way (probably the best way) to implement data structures. First of all we have to understand the concepts of lifetime, borrow  and ownership of Rust. 
* Note : This is not a rust tutorial
## Path
We will start with concepts pointers in C to understand references of Rust. Then we will visit to other fundamental concepts.

## <b> Rust References </b>
Here I try my best to come up with missing pieces of programming fundamentals unknown to programmer (java programmers :) ) In java like programming language we rarely care about memory management. 

### <u><b> Fundamentals Of Memory Allocation </b></u>: 
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

### <u>Memory leaks</u> :

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





## Implemented data structures:
I am going to implement following data structures and algorithms. I won't be explaining those. Code has explanations why it is done that way. I have kept the explanation as simple as possible. Linked list based data structure is a good starting point.

> Linked lists were a great innovation for simple computer architectures similar to the 1970-era PDP-11, but they defeat most of the hardware acceleration features of modern architectures with their parallel and pipelined execution resources. Multi-level memory caches, single- and multi-issue instruction pipelines, look-aside address translation and branch prediction caches, parallel arithmetic and logical execution units, etc. all end up stalling or being massively under-utilized when chasing linked lists. Think about false sharing while using linked list

* Follow the below sequence
* Stack : Linked list based stack my_stack/my_first_stack.rs

