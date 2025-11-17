fn main() {
 // Create an empty vector "City"
 let mut city: Vec<String> = Vec::new();
 // Print City Vector
 println!("(The city vector has element {})", city.len());
 // Push new elements into
 let mut Input = String::new();
 println!("(How many Cities do you want to enter?)");
 std::io::stdin().read_line(&mut Input).expect("Failed to read input");
 let city_num: i32 = Input.trim().parse().expect("Invalid input");
 for count in 0..city_num {
 let mut Input2 = String::new();
 println!("Enter City {}", count + 1);
 std::io::stdin().read_line(&mut Input2).expect("Failed to read input");
 let new_city: String = Input2.trim().parse().expect("Invalid input");
 city.push(new_city);
 }
 // print ("Your preferred cities are:");
 let mut count=1;
 // loop to iterate elements in vector
 for i in city {
 // Iterating through i on the vector
  println!("{} {}", count, i);
  count += 1;
 }
}