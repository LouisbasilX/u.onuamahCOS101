use std::collections::HashMap;

fn main(){
    let sum_average = stat([2,1,3,3,1].to_vec(), [450_000, 1_500_000, 750_000, 2_850_000, 250_000].to_vec());
    let sum = sum_average.get("sum").unwrap();
    let average = sum_average.get("average").unwrap();
    println!("The sum of sales records is N{} and the average is N{}", sum, average);
}

fn stat(freq :Vec<i32>, val :Vec<i32>) -> HashMap<String, i32> {
    let mut freq_sum = 0; 
    let mut val_sum  = 0;
    
    for  i in 0..freq.len() {
            freq_sum += freq[i];
            val_sum  +=  val[i];
     }

    let mut map = HashMap::new();
    map.insert("sum".to_string(), val_sum);
    map.insert("average".to_string(), val_sum/freq_sum);
    map
}