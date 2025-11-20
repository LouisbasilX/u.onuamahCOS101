use std::io;
use std::collections::HashMap;

fn main() {
   let mut level_map: HashMap<&str, Vec<&str>> = HashMap::new();
   level_map.insert("Academic", ["intern", "Research Assistant", "Phd Candidate", "Post-Doc Researcher", "Senior lecturer", "Dean"].to_vec());
   level_map.insert("Lawyer", ["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "partner"].to_vec());
   level_map.insert("Teacher", ["Placement", "Classroom", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"].to_vec()); 
   let positions = ["APS 1-2", "APS 3-5", "APS 6-8", "APS 8-10", "EL1 8-10", "EL2 10-13", "SES"];
   let offices = ["intern", "Administrator", "Senior Adminstrator", "Office Manager", "Director", "CEO"];
   loop{
    let name = read_input("Enter user-name");
    let job = collect_choice(["Academic", "Lawyer", "Teacher"].to_vec(), "Enter a Valid Occupation from the choices please");
    let mut level = collect_choice(level_map.get(job).unwrap().to_vec(), "Please choose a valid Level");
     
    let level_index:usize = index_of(&level_map.get(job).unwrap(), level);
    level = level_map.get(job).unwrap().to_vec()[level_index];
    
    println!("Dear {} A/an {} of level {} holds APS position {} and office {}", name, job, level, positions[level_index], offices[level_index]);
    
    let to_continue = read_input("Do you want to continue enter yes or no");
    if to_continue.trim().to_uppercase() == "NO".to_string(){
      break
    }
   }
}


fn read_input(text: &str) ->String{
    let mut res = String::new();
    println!("{}", text);
    io::stdin().read_line(&mut res).expect("Invalid");
    res
}
fn collect_choice<'a>(options:Vec<&'a str>, custom_text: &'a str) -> &'a str{
    let len = options.len() as i32;

    println!("{} \n Below are your options", custom_text);
    for  i in 0..len{
        println!("{}: {}", i + 1, options[i as usize])
    }
    let choice: String = get_valid_response((format!("Enter a valid choice from 1 to {}", len)).as_str(), |res, len| match res.trim().parse::<i32>() {
    Ok(val) if val > 0 && val < len + 1 => true,
    _ => false}, len).trim().parse().expect("Invalid Input");
    let choice_index: i32 = choice.trim().parse().expect("invalid");
    options[(choice_index as usize) - 1]
}
fn get_valid_response<F>(text:&str, condition: F, valid_num: i32)->String
where
     F: Fn(&str, i32) -> bool,
{
 let mut ans: String = String::new();
    loop{
       let res = read_input(text);
       if condition(&res.as_str(), valid_num){
        ans = res;
        break;
       }
       else {
        println!("Invalid Value entered Please re-enter");
       }
     }
  ans
}
fn index_of(v: &Vec<&str>, target: &str) -> usize {
    v.iter().position(|&x| x == target).expect("Value not found")
}