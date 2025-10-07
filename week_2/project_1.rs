fn main(){
  println!("The compound Interest is N {:.2}", compound_interest(520_000_000.0, 10.0, 5.0));
}

fn compound_interest(principal :f64, rate :f64, n :f64)-> f64 {
           let amount = principal * (1.0 + (rate/100.0)).powf(n);
           amount - principal
}