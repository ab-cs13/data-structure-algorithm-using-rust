
//we can have multiple owner of a string literals because string literals are immutable
/*
   ---Memory Representation---
                format is <starting address | length> 
                _____________
                | 100AF | 5 |  s1 : & str  (s1 also stays in stack) 
                |_______|___|
                   | 2000AE            ___________
                   |__________________|100AF | 5  | s2 :& str = s1 (s2 also stays in stack)
                   |                  |___________| 
    _______________|                     8000AF
   _|___________________________________
  | H | E | L | L | O |                 |
  |___|___|___|___|___|_________________|  stack frame
  100AF

  Address of s1 and s2 both are different. They holds the address of string literal. 
  No copy is created because string literals are immutable.
*/ 
#[test]
fn foo_01(){
    let s1:& str = "Hello"; 
    let s2:& str = s1;
    println!("s1 content :{},s1 address{:p}, string literal address:{:p}",s1, & s1, s1);
    println!("s2 content :{},s2 address{:p}, string literal address:{:p}",s2, & s2, s2) ;
}

/*
Though 'i' is mutable but value is immutable. So when we perform j=i, value gets copied to the memory location assigned to j. 
Similarly though s1 is mutable but it holds the address of an immutable element. Therefore, when perform s1=s2, ownership transfer 
never happens.
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

/*
#[test]
fn foo_03(){
  let s1:String = String::from("Hello Rust"); //String in rust is mutable and memory is allocated from heap. Therefore, can't be copied.
  let s2:String = s1; //When we perform the assignment s2=s1, ownership is transferred.
  println!("s2:{}",s2);
  println!("s1:{}",s1);//After move / transfer of ownership s1 become uninitialized. Hence accessing s1 generates compile time error
}
*/

/*
foo_10 will result compilation error.
#[test]
fn foo_10(){
    let mut s1:&str = "Hello";
    let ref_1: & mut (&str) = & mut s1; 
    let ref_2: & mut (&str) = & mut s1;
    println!("ref_1:{}",ref_1);
    println!("ref_2:{}",ref_2);
  }
*/