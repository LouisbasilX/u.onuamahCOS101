// Method to print the get value
fn value(n:Option<&char>)
{
 println!("Element of vector {:?}",n);
}
fn main() {
 let v = vec!['R','U','S','T','A','C','I','A','N'];
 let mut input = String::new();
 println!("Enter an index value btw (0 - 8)");
 std::io::stdin().read_line(&mut input).expect("Failed to read input");
 //index is the non negative value which is smaller than the size of the vector
 let index:usize = input.trim().parse().expect("Invalid input");
 // getting value at given index value
 let ch: Option<&char> = v.get(index);
 value(ch);
}