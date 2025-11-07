use std::io;
use std::collections::HashMap;
use std::env::Args;


fn main() {
   let menu_map:HashMap<&str, i32> = 
        [("P", 3200), ("F", 3000), ("A", 2500), ("E", 2000), ("W", 2500)]
        .iter()
        .cloned()
        .collect();
    take_orders(menu_map);    
}

fn add_customer() -> String{
   println!("Add a customer to proceed");
   let customer_name = read_input("Enter the customer name");
   customer_name
}

fn read_input(text: &str) ->String{
    let mut res = String::new();
    println!("{}", text);
    io::stdin().read_line(&mut res).expect("Invalid");
    res
}

fn display_menu(){
     let see_menu = read_input("Do you want to see the menu?. \n enter y to view menu or n to order now. \n any other input would mean proceed");
      if see_menu.trim().to_uppercase().as_str() == "Y"{
            println!("Menu			           Price\n
     P = Pounded Yam/Edinkaiko Soup  	- N3,200\n
     F = Fried Rice & Chicken 			- N3,000\n
     A = Amala & Ewedu Soup		        - N2,500\n
     E = Eba & Egusi Soup			    - N2,000\n
     W = White Rice & Stew			    - N2,500
          ");
    }
}

fn take_orders(menu_map: HashMap<&str, i32>){
    loop{
      let customer = add_customer();
      let mut total_bill = 0;
      loop{
        display_menu();
        println!("Enter a food code to place orders {}", customer);
      let food_code = get_valid_response(
        "Enter a food code in the menu",
       |code, menu_map| {
        let code_up = code.trim().to_uppercase();
        menu_map.contains_key(code_up.as_str())
         },
       &menu_map,
      ).trim().to_uppercase();
       
        let show_price = read_input("Enter y to see price or n to proceed");
         
        if show_price.trim().to_uppercase().as_str() == "Y"{
           println!("the Item of code {} costs {}", food_code, menu_map.get(&food_code.as_str()).unwrap());
        }
       
        let price = menu_map.get(food_code.as_str()).unwrap();
        let quantity: i32 = get_valid_response("Enter a valid quantity", |quantity, menu_map| match quantity.trim().parse::<i32>() {
    Ok(val) if val > 0 => true,
    _ => false}, &menu_map).trim().parse().expect("Invalid Input");
      
        total_bill += price * quantity;
        let to_continue = read_input("Do you want to continue? \n Enter y to order more or n to end");
         if to_continue.trim().to_uppercase() == "N".to_string(){
            break
          }
        }
         println!("{}! Your total bill is {}", customer ,total_bill);
         let to_continue = read_input("Do you want to continue? \n Enter y to order more or n to end");
          if to_continue.trim().to_uppercase() == "N".to_string(){
            break
          }

      }  
   }

fn get_valid_response<F>(text:&str, condition: F, menu_map: &HashMap<&str, i32>)->String
where
     F: Fn(&str, &HashMap<&str, i32>) -> bool,
{
 let mut ans: String = String::new();
    loop{
       let res = read_input(text);
       if condition(&res.as_str(), &menu_map.clone()){
        ans = res;
        break;
       }
       else {
        println!("Invalid Value entered Please re-enter");
       }
     }
  ans
}

