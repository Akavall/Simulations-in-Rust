pub fn fact(n: i32) -> f64 {
    
    let mut prod = 1.0;
    for i in (1..(n+1)) {
       let f = i as f64; 
       prod = prod * f;
    }
        
   return prod;
}
