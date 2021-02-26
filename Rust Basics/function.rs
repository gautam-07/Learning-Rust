// Functions in rust
// Rust uses snake case as default

// Rust does not care about the order in which functions are defined, its just that they are defined somewhere in the source code.


fn another_function(){
   println!("Another function in rust");
}


// Passing parameters to a function in rust
fn another_function2(x:i32){
   println!("The value of x is {}",x);
}

// Functions returning values
// In rust we use an arrow -> to specify the return value 

fn another_function3(x:i32, y:i32) -> i32{
   let mut b = x + y;
   b = b * 100;
   return b;
}

// Main function in rust where the execution starts
fn main(){
   println!("Hello World");
   another_function();
   another_function2(56);

   let a = another_function3(50,50);
   println!("The sum of two numbers is: {}",a);
}