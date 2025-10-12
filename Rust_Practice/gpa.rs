use std::io;
use std::collections::HashMap;
use std::cmp;

fn conv_grade<'a>(grade :&String, scale :&HashMap<&str, i32>) -> i32{
  let upper = grade.to_uppercase();
  if scale.contains_key(&upper.as_str()) {
    return *scale.get(upper.as_str()).unwrap_or(&0);
  }
if let Ok(num) = grade.parse::<i32>() {
  if num < 45{
  return 0
  }
  if num > 100{
   return -1
   }
  let n = ((num as f64) / 10.0).floor() as i32;
  return cmp::min(n - 2, 5);
 }
-1
}

fn calc_gpa(points :Vec<String>, grades :Vec<String>) -> f64 {
let scale: HashMap<&str, i32> = 
        [("A", 5), ("B", 4), ("C", 3), ("D", 2), ("F", 0)]
        .iter()
        .cloned()
        .collect();
  let mut total_points = 0;
  let mut score = 0;
  for i in 0..points.len(){
     total_points += points[i].parse::<i32>().unwrap();
     score += points[i].parse::<i32>().unwrap() * conv_grade(&grades[i], &scale)
   }
   score as f64 / total_points as f64
  }
  
fn main() {
    // Create a mutable string to hold input
    let mut name = String::from("");

    println!("Enter your name: ");
    
    // Read input from user
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Remove trailing newline
    name = name.trim().to_string();
    
    println!("Hello, {}! this will help u calculate your GPA but first I need your grade points and grades in the order: Gradepoint Grade, ie 2 A,3 B or 2 70,3 60 so below enter your course points and grades in the valid format to get your GPA score :", name);
    let mut data = String::from("");
  loop{ 
      data.clear();
       io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");
     
     data = data.trim().to_string();
     if validate_data(&data) {
      break;
    }
    else {println!("{} please enter your grade point and grades in the correct format:\n Gradepoint1 grade1,Gradepoint2 grade2,\nfor ex: 2 A,3 B,4 C or 2 78,3 76,5 90", name);}
    }
  
     
   loop {
     let mut res = String::from("");
     println!("Is that all your courses? enter y to proceed or n to add more courses. an invalid input counts as proceed :");
         
     io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
        
        if res.trim().eq_ignore_ascii_case("y"){
           break;
           }
        else if res.trim().eq_ignore_ascii_case("n"){
             let mut more_data = String::from("");
             println!("add more courses in the same format");
        
          loop{   
                  more_data.clear();
                  io::stdin()
        .read_line(&mut more_data)
        .expect("Failed to read line");
                  more_data = more_data.trim().to_string();
              if validate_data(&more_data) {
                 break;
              }
             else {println!("{} please enter your grade point and grades in the correct format:\n Gradepoint1 grade1,Gradepoint2 grade2,\nfor ex: 2 A,3 B,4 C or 2 78,3 76,5 90", name);
                }
           
              }
            data.push_str(",");
            data.push_str(more_data.trim());
         }
         else {
          println!("Invalid Input. Enter y to proceed or n to add more courses")
         }
     
     }

     let pg = get_gra(&data);
     println!("Your GPA is {:.2}", calc_gpa(pg[0].clone(), pg[1].clone()))
     
}
  fn get_gra(s :&String) -> Vec<Vec<String>>{
     let mut courses = s.split(",");
     let mut points = Vec::new();
     let mut grades = Vec::new();
     let mut res = Vec::new();
     while let Some(item) = courses.next(){
      let item = item.to_string();
        let pg: Vec<String> = item.split(" ").map(|s| s.to_string()).collect();
        points.push(pg[0].clone());
        grades.push(pg[1].clone());
    }
    res.push(points);
    res.push(grades);
    res
  }

  fn is_number_string(s: &String)-> bool{
     s.parse::<i32>().is_ok()
  }

 fn validate_data(data: &String)-> bool{
    let mut scores = data.split(",");
    while let Some(item) = scores.next(){
      let item = item.to_string();
      let scale: HashMap<&str, i32> = 
        [("A", 5), ("B", 4), ("C", 3), ("D", 2), ("F", 0)]
        .iter()
        .cloned()
        .collect();
      let pg: Vec<String> = item.split(" ").map(|s| s.to_string()).collect();
      let is_invalid = pg.len() < 2 || !is_number_string(&pg[0]) || (!is_number_string(&pg[1]) && !scale.contains_key(&pg[1].as_str()));
     
       
      if is_invalid {
        return false;
      }
     
    }
     true
  }