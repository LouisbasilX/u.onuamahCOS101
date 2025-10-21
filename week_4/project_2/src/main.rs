use std::io;

fn main() {
    println!("Hello! I would determine your annual incentive from your experience and age");

    let mut input1 = String::new();
    println!("Enter Your Age: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let age :u32 = input1.trim().parse().expect("Invalid input"); 

    let mut input2 = String::new();
    loop{
        input2.clear();
        println!("Are You experienced ?. enter yes or no: ");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        if input2.trim().to_uppercase() == "YES" || input2.trim().to_uppercase() == "NO"{
            break;
        }
        println!("Sorry. you have entered something other than yes or no");
    }
   
    let mut is_experienced :bool = false;
    if input2.trim().to_uppercase() == "YES"{
        is_experienced = true;
    }
   
      println!("Given your age and experience status, your annual incentive is N{}", ann_inc(age, is_experienced))
}

fn ann_inc(age: u32, experience: bool) -> i32{
    if experience == false {
        return 100_000;
    }

    else if age >= 40{
        return 1_560_000;
    }

    else if age >= 30 && age < 40{
        return 1_480_000;
    }

    else {
        return 1_300_000;
    }
}
