# data-structure-algorithm-using-rust
To learn a language: implement data structures. Therefore, to understand the concepts of lifetime, borrow  and ownership of Rust, I am writing basic data structure. 
* Note : This is not a rust tutorial
## Path
We will start with concepts pointers in C to understand references of Rust. Then we will visit to other fundamental concepts.

## Rust References
Here I try my best to come up with missing pieces of programming fundamentals unknown to programmer (java programmers :) ) In java like programming language we rarely care about memory management. 

### Fundamentals Of Memory Allocation
* Stack and Heap : Both stack and heaps are parts of memory available to program to use during runtime. If the size of the data is known during compile time, memory is allocated from stack. If size of the data is unknown during compile time it is preferred (In C we can allocate memory from stack using alloc system call !!! )  to allocate memory from heap and starting address of heap memory is returned as pointer which stays in stack memory. It is really  easy to manage memory allocated from stack. When control goes outside the scope of 
a method or function, stack entry associated with it is popped and memory is reclaimed. But, there is a catch, if pointer holding an address from heap; thought the memory allocated to the pointer to store the address of the heap is reclaimed by popping the stack entry, but the memory allocated in heap stays there. OS never follows the pointer to reclaim the memory from heap. Then why we allocate memory from heap ? 
1: If size is unknown
2: Memory allocated from stack for those data whose size is known at compile time and size is never going to change. When these data passed to different function as arguments (Note : what is the difference between parameter and argument ?) copy of the data is passed which a completely different memory address. And we call this technique as " Call by Value ". Here each variable going to have their own copy of data. When stack entry is popped, memory allocated to these variables are reclaimed. But, story is different for memory allocated from heap. We can't copy the content of heap when we pass the data as an argument to a method /function call. Because the size of the content is unknown. It won't be performant also.         



## Implemented data structures:
I am going to implement following data structures and algorithms. I won't be explaining those. Code has explanations why it is done that way. I have kept the explanation as simple as possible. Linked list based data structure is a good starting point.

> Linked lists were a great innovation for simple computer architectures similar to the 1970-era PDP-11, but they defeat most of the hardware acceleration features of modern architectures with their parallel and pipelined execution resources. Multi-level memory caches, single- and multi-issue instruction pipelines, look-aside address translation and branch prediction caches, parallel arithmetic and logical execution units, etc. all end up stalling or being massively under-utilized when chasing linked lists. Think about false sharing while using linked list

* Follow the below sequence
* Stack : Linked list based stack my_stack/my_first_stack.rs

