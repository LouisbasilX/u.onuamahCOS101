use std::io;

fn main() {
    println!("This Command line will solve any quadratic equation given the coefficients a , b and c");
    let mut input1   = String::new();
    println!("Enter the Value for a: ");
    io::stdin().read_line(&mut input1).expect("invalid input");
    
    let mut input2  = String::new();
    println!("Enter the Value for b: ");
    io::stdin().read_line(&mut input2).expect("invalid input");

    let mut input3  = String::new();
    println!("Enter the Value for c: ");
    io::stdin().read_line(&mut input3).expect("invalid input");

    let a: f64 = input1.trim().parse::<f64>().expect("invalid Input"); 
    let b: f64 = input2.trim().parse::<f64>().expect("invalid Input");
    let c: f64 = input3.trim().parse::<f64>().expect("invalid Input");

    let det = det(a, b, c);
    
    if det < 0.0 {
        println!("Unfortunately the equation has no real roots \n the roots are (-{} + (√({}))) / {} and (-{} - (√({}))) / {}", b, det, 2.0 * a, b, det, 2.0 * a);
        return;
    }
    let root1 = (-b + det.sqrt())/ (2.0 * a);
    let root2 = (-b - det.sqrt())/ (2.0 * a);

    println!("The roots are: \n X1 = {:.3} \n X2 = {:.3}", root1, root2);
}

fn det(a: f64, b :f64, c :f64) -> f64{
      (b * b) - (4.0 * a * c)
}