use std::io;

fn main() {
   let mut chosen_dev = String::new();
   let mut level_of_experience:i32 = 0;
    
    loop{
       let current_dev = read_input("Enter developer name");
       let experience = get_valid_number("please enter the level of experience in years");
       if experience > level_of_experience {
        level_of_experience = experience;
        chosen_dev = current_dev;
       }
       let to_continue = read_input("Do you wish to continue ?. Enter yes or no");
       if to_continue.to_uppercase() == "NO".to_string(){
        break;
       }
    }
 
     println!("the chosen developer is {} and he has {} years of experience", chosen_dev, level_of_experience);
}

fn read_input(text: &str) ->String{
      println!("{}", text);
      let mut res = String::new();
      io::stdin().read_line(&mut res).expect("invalid");
      res.trim().to_string()
}
fn get_valid_number(text: &str) -> i32{
    let mut ans = String::new();
    loop{
        let mut res = read_input(text);
        match res.trim().parse::<i32>() {
         Ok(num) => {
            ans = res;
            break;
         }, 
          Err(_) => println!("You have entered an invalid number. Please re-enter"),
       }
        
    }
    let res = ans.trim().parse::<i32>().expect("Invalid");
  res
}