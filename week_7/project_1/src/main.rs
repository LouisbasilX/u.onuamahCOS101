use std::io;
use std::collections::HashMap;
use std::env::Args;

struct Equation {
    func: Box<dyn Fn(Vec<f64>) -> f64>,
    params: Vec<&'static str>,
}

fn main() {
   let mut equation_map: HashMap<&str, HashMap<&str, Equation>> = HashMap::new();
   
   let mut area_map: HashMap<&str, Equation> = HashMap::new();

   area_map.insert("Trapezium", Equation{
    func: Box::new(|args: Vec<f64>| (args[0] + args[1])/args[2]),
    
    params: vec!["shorter parallel side", "Longer Parallel side", "Height"],
   }
   );
   area_map.insert("Circle", Equation {
        func: Box::new(|args: Vec<f64>| std::f64::consts::PI * args[0].powi(2)),
        params: vec!["radius"],
    });
    area_map.insert("Rectangle", Equation {
        func: Box::new(|args: Vec<f64>| args[0] * args[1]),
        params: vec!["length", "width"],
    });
    area_map.insert("Triangle", Equation {
        func: Box::new(|args: Vec<f64>| 0.5 * args[0] * args[1]),
        params: vec!["base", "height"],
    });
      area_map.insert("Rhombus", Equation {
        func: Box::new(|args: Vec<f64>| args[0] * args[1]),
        params: vec!["base", "height"],
    });
     area_map.insert("Parallelogram", Equation {
        func: Box::new(|args: Vec<f64>| args[0] * args[1]),
        params: vec!["base", "height"],
    });
     area_map.insert("Sphere", Equation {
        func: Box::new(|args: Vec<f64>| 4.0 * std::f64::consts::PI * args[0].powi(2)),
        params: vec!["radius"],
    });
    area_map.insert("Cuboid", Equation {
        func: Box::new(|args: Vec<f64>| 2.0 * (args[0] * args[1] + args[2] * args[0] + args[1] * args[2])),
        params: vec!["length", "width", "height"],
    });
      area_map.insert("Cube", Equation {
        func: Box::new(|args: Vec<f64>| 6.0 * (args[0].powi(2))),
        params: vec!["length"],
    });
     area_map.insert("Cylinder", Equation {
        func: Box::new(|args: Vec<f64>| 2.0 * std::f64::consts::PI * args[0]*(args[1] + args[0])),
        params: vec!["radius", "height"],
    });
   
    equation_map.insert("Area", area_map);

    // --- VOLUME formulas ---
    let mut volume_map: HashMap<&str, Equation> = HashMap::new();
    volume_map.insert("Sphere", Equation {
        func: Box::new(|args: Vec<f64>| (4.0 / 3.0) * std::f64::consts::PI * args[0].powi(3)),
        params: vec!["radius"],
    });
    volume_map.insert("Cuboid", Equation {
        func: Box::new(|args: Vec<f64>| args[0] * args[1] * args[2]),
        params: vec!["length", "width", "height"],
    });
    volume_map.insert("Cylinder", Equation {
        func: Box::new(|args: Vec<f64>| std::f64::consts::PI * args[0].powi(2) * args[1]),
        params: vec!["radius", "height"],
    });
    volume_map.insert("Cube", Equation {
        func: Box::new(|args: Vec<f64>| args[0].powi(3)),
        params: vec!["length"],
    });
    volume_map.insert("Cylinder", Equation {
        func: Box::new(|args: Vec<f64>| (1.0/ 3.0) * std::f64::consts::PI * args[0].powi(2) * args[1]),
        params: vec!["radius", "height"],
    });
     volume_map.insert("Triangular Prism", Equation {
        func: Box::new(|args: Vec<f64>| (1.0/3.0) * args[0] * args[1] * args[2]),
        params: vec!["base length", "base width", "height"],
    });
    equation_map.insert("Volume", volume_map);

    println!("Welcome to the shape calculator");

    loop{
        calculate_area_volumes(&equation_map);
        let to_continue = read_input("Do you want to continue?\n Enter y to continue or n to end here.");
        if to_continue.trim().to_uppercase() == "N"{
            break;
        }
    }
}
fn calculate_area_volumes(equation_map: &HashMap<&str, HashMap<&str, Equation>>) -> f64{
     let property = collect_choice(vec!["Area", "Volume"], "Please Select the property you want to get \n enter a number to select a choice");
     let prop = equation_map.get(property);

     let shapes = prop.expect("Invalid").keys().cloned().collect();
     let shape_choice = collect_choice(shapes, "Please select the shape you want \n Select from the available ones with a number");
     
     let shape_struct = prop.expect("Invalid").get(shape_choice);
     let mut args : Vec<f64> = Vec::new(); 

     for param in &shape_struct.unwrap().params {
    
        let val_text = "Enter a valid ".to_owned() + param;
        let val_str = get_valid_response(
        &val_text.as_str(),
        |res, _| res.trim().parse::<f64>().map(|v| v > 0.0).unwrap_or(false),
        0,
    );
        let val = val_str.trim().parse::<f64>();
        args.push(val.expect("invalid"));
     }
     let calculator = &shape_struct.unwrap().func;
     let ans = calculator(args);
     println!("Your answer = {}", ans);
     ans
}
fn read_input(text: &str) ->String{
    let mut res = String::new();
    println!("{}", text);
    io::stdin().read_line(&mut res).expect("Invalid");
    res
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
