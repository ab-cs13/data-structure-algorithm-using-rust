
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

//clone() performs deep copy for String implementation. Copies entire content to a new memory location. Therefore, move doesn't happen
//for source
#[test]
fn foo_04(){
  let s1:String = String::from("Hello");
  let s2:String = s1.clone();
  println!("s1:{}",s1);
  println!("s2:{}",s2);
}
/*
#[test]
fn foo_05(){
  let mut i:i32 = 5; //mutable variable
  //constant reference to a mutable variable. but variable is referred immutably hence reference can't change the value of i
  let ref_1: & i32 = & i; 
  println!("{}",ref_1);
  //variable is referred immutably hence reference can't change the value of i
  (*ref_1) = 6; //generates compilation error
}
*/
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
#[test]
fn bar(){
  let mut s = String::from("Hello");
  let r1 = &s;
  let _r2 = &s;
  println!("{}",r1)
}

/*
foo_10 will result compilation error. We can't have two '& mut reference' for a variable and keep using them both. 
#[test]
fn foo_10(){
    let mut s1:&str = "Hello";
    let ref_1: & mut (&str) = & mut s1; 
    let ref_2: & mut (&str) = & mut s1;
    println!("ref_1:{}",ref_1);
    println!("ref_2:{}",ref_2);
  }
*/
#[test]
fn test_12(){
  let mut s=String::from("Hello");
  {
      let r1 = &s;
      println!("{}",r1);
  }
  {
    let r2 = & mut s;
    r2.push('!');
    println!("{}",r2);
  }
  let r3 = & s;
  let r4 = & s;
  println!("{}",r3);
  println!("{}",r4);
  println!("{}",& mut s);
  println!("{}",& s);
}
#[test]
fn test_13() {
  let mut i:i32 = 3;
  {
      let r1 = & i;
      println!("{}",r1);
  }
  {
      let r2 = & mut i;
      (*r2) = 5;
      println!("{}",r2);
  }
  let r3 = & i;
  println!("{}",r3);
}