/*
Slice concept in Rust
 
A string slice is reference to a part of a string
We can create slices using range within brackets.

Ownership, borrowing, and slices ensure memory safety

*/

fn main(){
   let s = String::from("Hello, World!");

   let hello = &s[0..5];

   println!("{}",hello);
}