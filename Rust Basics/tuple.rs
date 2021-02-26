// Tuple in rust is used to group data of different types.

fn main(){
   let tup: (i32, f64, u8) = (500, 3.5, 1);
   let (x,y,z) = tup;

   // Destructuring of tuple
   println!("Value of x = {}",x);
   println!("Value of y = {}",y);
   println!("Value of z = {}",z);

   // Another way of destructuring the tuple
   // By using the dot operator
   let a = tup.0;
   let b = tup.1;

   println!("Value of a = {} and b = {}", a,b);
}



