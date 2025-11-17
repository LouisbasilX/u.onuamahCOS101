fn main() {
 // Name vector
 let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Joe"];
 // Age vector
 let age = [16,19,19,22,20,21,18,23];
 println!("\\Age allocation:\\n");
 //loop to iterate elements in vector
 for i in 0..age.len() {
 // iterating through i on the vector
 println!("{} is {} years old", name[i], age[i]);
 }
}