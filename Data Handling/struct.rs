/*
Struct in Rust
1. Custom data types
2. Make up a meaningful group

Struct are similar to tuples
*/

fn main(){

   struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
   };

   let user = User{
      username: String::from("Gautam"),
      email: String::from("example@xyz.com"),
      sign_in_count: 1,
      active: true,
   };

   println!("{}",user.username);
   println!("{}",user.email);
   println!("{}",user.sign_in_count);
   println!("{}",user.active);

}