/*
Race Conditions in Rust
A data race is similar to race conditions and happens when these three behavior occurs:-

1. When two or more pointers access the same data at the same time.
2. At least one pointer is used to write the data.
3. There's no mechanism being used to synchronize access to the data.

Data races can cause undefined behavior and can be difficult to diagnose at run time.

Rust prevents this from happening because it won't even compile the code with data races.

Summary:-

   * At any given point of time you can have either but not both:-
      1. One mutable reference.
      2. Any number of mutable references.

   * References must always be valid

*/

fn main(){
   let refers_nothing = dangle();
   println!("{}",refers_nothing);
}

fn dangle() -> String {
   let s = String::from("Hello World");
   return s;
}