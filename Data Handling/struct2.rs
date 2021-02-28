// Calculating area of rectangle

struct Rectangle{
   width: u32,
   height: u32,
}

fn main(){
   let rect1 = Rectangle{width:10, height:20};

   println!("Area of rectangle is : {}",area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32{
   rectangle.width * rectangle.height
}