/**
 * Simple use case of enum to understand it. We are also going find out how enums in rust different from java enum.
 * Let's have a simple example. Suppose we have to setup our delivery address on an e-commerce portal. 
 * Portal required two address for delivery. Office address and Home address. Each of these address needs the postcode.
 * Postcode come as input from user determined during runtime.
 * And each type has specific time between which delivery can be expected. Let's first model it in java
 //Java way
 /**
 * Different address type. Each address type has specific time between which delivery can be expected.
 To keep things simple kept delivery times kept in String :) 
 */
 public enum AddressType {
    OFFICE("10 AM", "3 PM"), HOME("4 PM","7 PM");
    private final String deliveryStartTimeOfDay;
    private final String deliveryEndTimeOfDay;

    AddressType(String deliveryStartTimeOfDay, String deliveryEndTimeOfDay) {
        this.deliveryStartTimeOfDay = deliveryStartTimeOfDay;
        this.deliveryEndTimeOfDay = deliveryEndTimeOfDay;
    }

    @Override
    public String toString() {
        return "AddressType{" +
                "deliveryStartTimeOfDay='" + deliveryStartTimeOfDay + '\'' +
                ", deliveryEndTimeOfDay='" + deliveryEndTimeOfDay + '\'' +
                '}';
    }

 //Can we have the post code inside all enum constant of AddressType. Post code is an input from user determined
 //during runtime.
 //No we can't. Enums in java can accept parameters. But not during runtime. Parameters has to be defined during
 //compile time. For example expected delivery time for home address and office address. These two are constant and
 //defined during compile time.But, that is not the case with post code.That's why we need another class 
 //to store postcode during runtime.

 public class Address {
    private final String postCode;
    private final AddressType addressType;

    public Address(String postCode, AddressType addressType) {
        this.postCode = postCode;
        this.addressType = addressType;
    }

    public String getPostCode() {
        return postCode;
    }

    public AddressType getAddressType() {
        return addressType;
    }

    @Override
    public String toString() {
        return "Address{" +
                "postCode='" + postCode + '\'' +
                ", addressType=" + addressType +
                '}';
    }
}

We can model above implementation exactly in same way in Rust. class is can be replaced by struct.
And Java enum can be replaced by Rust enum. But, Rust enum is different. Rust enum can accept arguments during runtime. 
Actually we don't an additional struct definition in Rust to store postcodes.   
 */
pub enum Address<'a>{
    /**
     * 1 : postcode
     * 2 : start time
     * 3 : end time
     */
    //We can't have OFFICE(String,String,String) why ?? When we pass reference Address enum to foo_delivery_service (we can't pass the
    //enum variable to avoid move or transfer of ownership), and try to use postcode or other attributes, as those are not reference
    //move or transfer ow ownership will happen. Attributes will be uninitialized and compile will complain. 
    //Refer commented example foo_enum_message
    HOME(& 'a String,& 'a String,& 'a String),
    OFFICE(& 'a String,& 'a String,& 'a String),
   
}

#[test]
fn foo_enum_1(){
    //constant values can be kept some where else
  let home_postcode :String = String::from("ABCS13");  
  let home_delivery_start_time = String::from ("4 PM");
  let home_delivery_end_time = String::from ("7 PM");
  let home_address : Address = Address ::HOME(& home_postcode, & home_delivery_start_time, & home_delivery_end_time);
  let office_postcode :String = String::from("QWE45");  
  let office_delivery_start_time = String::from ("9 AM");
  let office_delivery_end_time = String::from ("3 PM");
  let office_address : Address = Address ::OFFICE(& office_postcode, & office_delivery_start_time, & office_delivery_end_time);
  //depending on address type let's call a function to deliver items. 
  //Delivery service for home and office is different.
  foo_delivery_service(& home_address);
  foo_delivery_service(& office_address);
}
fn foo_delivery_service(address:& Address){
    //If home address, should be delivered by blue dart delivery service
    if let & Address::HOME(postcode,start ,end )=address{
        println!("Delivered by Blue Dart : {}, between {} to {}",postcode,start,end);
    }
    //If office address, delivered by DHL
    if let & Address::OFFICE(postcode,start ,end )=address{
        println!("Delivered by DHL : {}, between {} to {}",postcode,start,end);
    }
}
/*
//With String variable
error[E0507]: cannot move out of `msg` as enum variant `HELLO` which is behind a shared reference
   --> src/hello_enum.rs:119:45
    |
119 |   if let & Message::HELLO(string_message) = msg{
    |          --------------------------------   ^^^
    |          |                |
    |          |                data moved here
    |          |                move occurs because `string_message` has type `String`, which does not implement the `Copy` trait
    |          help: consider removing the `&`: `Message::HELLO(string_message)`

error[E0507]: cannot move out of `msg` as enum variant `GOOD_BYE` which is behind a shared reference
   --> src/hello_enum.rs:122:48
    |
122 |   if let & Message::GOOD_BYE(string_message) = msg{
    |          -----------------------------------   ^^^
    |          |                   |
    |          |                   data moved here
    |          |                   move occurs because `string_message` has type `String`, which does not implement the `Copy` trait
    |          help: consider removing the `&`: `Message::GOOD_BYE(string_message)`

For more information about this error, try `rustc --explain E0507`.
warning: `code` (bin "code" test) generated 7 warnings
error: could not compile `code` due to 2 previous errors; 7 warnings emitted
pub enum Message{
    HELLO(String),
    GOOD_BYE(String),
}
#[test]
fn foo_enum_message(){
    let msg1:Message = Message::HELLO(String::from("Hello Rust"));
    let msg1:Message = Message::GOOD_BYE(String::from("Good bye others !!!"));

}
fn bar_message(msg:& Message ){
  if let & Message::HELLO(string_message) = msg{
    println!("println! prints {}",string_message)
  }
  if let & Message::GOOD_BYE(string_message) = msg{
    println!("System.out.println used to print {}",string_message)
  }
}
*/

/*
-------------------------------------------------------------------

Always pass reference as argument and return variable from function.
But you can always return a reference from a method if data the 
referred by the reference belongs to struct. The single factor always
need to be considered while returning reference "Rust never allows 
dangling references."

--------------------------------------------------------------------

*/