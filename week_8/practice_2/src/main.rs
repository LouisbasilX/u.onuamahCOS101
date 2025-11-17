fn main() {
 let v = vec!['C', 'O', 'M', 'P', 'U', 'T','E','R'];
 let mut input = String::new();
 println!("Enter an index value (0 - 8):");
 std::io::stdin().read_line(&mut input).expect("Failed to read line;");
 //index is the non negative value which is smaller than the size of the vector
 let index = input.trim().parse::<usize>().expect("invalid");
 //getting value at given index value
 let ch: char = v[index];
 println!("{} is the character for index {}", ch, index);
}