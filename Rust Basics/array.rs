// Array in rust

// 1. A collection of multiple values
// 2. Every element must have the same type
// 3. Arrays in rust have a fixed length


fn main(){
   let months = ["January", "February", "March", "April", "May", "June", "July",    "August", "September", "October", "November", "December"];

   println!("First   month = {}", months[0]);
   println!("Second  month = {}", months[1]);
   println!("Third   month = {}", months[2]);
   println!("Fourth  month = {}", months[3]);
   println!("Fifth   month = {}", months[4]);
   println!("Sixth   month = {}", months[5]);
   println!("Seventh month = {}", months[6]);
   println!("Eighth  month = {}", months[7]);
   println!("Ninth   month = {}", months[8]);
   println!("Tenth   month = {}", months[9]);
   println!("Eleven  month = {}", months[10]);
   println!("Twelve  month = {}", months[11]);

   // Rust do bounds checking automatically unlike languages C and C++;
}