/*
In other programming languages
1. Pointers and references : C and C++
2. Manually managing pointers is dangerous
3. Memory corruption issues
4. Java, Python, Ruby use garbage collector



Ownership in Rust
Rules:-

1. Each variable has a value and the variable  itself is called its owner
2. There can only be one owner at a time
3. When the owner gets out of scoped, the value will be dropped. 

*/

fn main(){
   let mut s1 = String::from("Hello");
   
   s1.push_str(", World!");
   let s2 = s1;

   println!("{}",s2);
}

/*
It is done because Rust calls a drop function and cleans up the heap memory but if two variables are pointing to the same location then it causes an error called "double free error" 

The process of transferring s1 to s2 is called "move".As soon as s2 is created s1 goes out of scope. 

This is called shallow copy in other programming languages
If we want to deeply copy the heap data along with the stack data then we use a common method called "clone".

*/





