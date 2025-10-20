use std::io;

fn main(){
    println!("\n Student Information Management System!");
    
    //Input Name
    println!("\n Please Enter your name. ");
    let mut name = String::new();

       io::stdin().read_line(&mut name).expect("Failed to read input");

       println!("Your Name is {}", name);


       //input Age
     println!("Enter your Age");
     let mut age = String::new();
     io::stdin().read_line(&mut age).expect("Failed to read Input");

     let age:i32 = age.trim().parse().expect("Input is not a valid Integer");
     println!("Your age is: {}", age);
}