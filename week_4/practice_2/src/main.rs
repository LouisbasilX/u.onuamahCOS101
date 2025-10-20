use std::io;

fn main() {
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();

loop{
  input1.clear();
  input2.clear();
  input3.clear();
  println!("Enter first edge of triangle");
  io::stdin().read_line(&mut input1).expect("Not a valid string");
  let a:f32 = input1.trim().parse().expect("Not a valid number");

  println!("Enter Second edge of triangle");
  io::stdin().read_line(&mut input2).expect("Not a valid string");
  let b:f32 = input2.trim().parse().expect("Not a valid number");

   println!("Enter Third edge of triangle");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let c:f32 = input3.trim().parse().expect("Not a valid number");
   
   if a + b > c && b + c > a && a + c > b {
    break;
   }
   
   println!("You have unfortunately entered the dimensions of an imaginary triangle\n
   please re-enter values");
}
let a:f32 = input1.trim().parse().expect("not a number");
let b:f32 = input2.trim().parse().expect("not a number");
let c:f32 = input3.trim().parse().expect("not a number");



let s:f32 = (a + b + c) / 2.0;
let mut area:f32 = s * (s - a) * (s - b) * (s - c);
area = area.sqrt();

   println!("Area of the triangle = {:.2} units squared", area);

}
