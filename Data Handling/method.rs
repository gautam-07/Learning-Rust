/*
Methods in Rust;

They are similar to functions;
Declared with the keyword "fn";

Methods are different from functions as they are defined within the context of struct or an enum 

And their first parameter is always "&self" which represents the instance of the struct on  which the method is being called on; 

To define a function within the context of a struct we start with an "impl" block;
*/

struct Rectangle {
   width: u32,
   height: u32,
}

impl Rectangle {
   fn area(&self) -> u32 {
      return self.width * self.height;
   }
}

fn main(){

   let rect = Rectangle {
      width: 10,
      height: 20,
   };

   println!("Area of rectangle is : {}", rect.area());
}

