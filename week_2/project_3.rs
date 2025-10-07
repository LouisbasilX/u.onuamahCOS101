fn main(){
    println!("The value of the TV after 3 years is N {:.2}", compound_amount(510_000.0, -5.0, 3.0));
}

fn compound_amount(principal :f64, rate:f64, n:f64) -> f64{
    principal * (1.0 + (rate / 100.0)).powf(n)
}
