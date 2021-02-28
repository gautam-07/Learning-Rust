/*
References and Borrowing in Rust

The semantics of passing a value to a function is similar to assigning values to variables.
Passing a variable to a function will either move or copy just like assignment



Concept of references:-
What if we want to let a function use a value but not take ownership. Rust provides this facility with references.
Use "&" sign at passing and receiving values


If we want to change the value of the variable that we have borrowed for that the type must be mutable.
And while calling and receiving the function you have to write it like this

                        takes_ownership(&mut s);
                        fn takes_ownership(some_string: &mut String){
                           //some code
                        }

While dealing with integers rust does a copy operation whereas while dealing with strings which are allocated on the heap it makes the move operation

But mutable references has one big restriction - You can only have one mutable reference to a particular piece of data in a particular scope.
This restriction allows for mutation but in a very controlled manner.
Benefit :- It prevent data races at compiling.
*/

fn main(){
   let s = String::from("Hello");

   println!("{}",s);
   takes_ownership(&s);

   let x = 5;
   makes_copy(x);

}

fn takes_ownership(some_string: &String){
   println!("{}, World!",some_string);
}

fn makes_copy(some_integer: i32){
   println!("{}",some_integer);
}