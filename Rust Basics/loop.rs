// Loops in rust 


// Three ways you can loop 
// 1. Using "loop" keyword
// 2. Using "while" loop
// 3. Using "for" loop


fn main(){
   let mut a = 3;

   let b = [1,2,3,4,5];

   loop{
      println!("Looping....");
      break;
   }

   while a != 0 {
      println!("Looping....");
      a = a - 1;
   }

   // using ".iter()" method to make it an iterable object

   for element in b.iter(){
      println!("Looping....");
   }
}