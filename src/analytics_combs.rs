use math_tools;

pub fn calc_expected_n_crosses_combinations(prob_down: f64, n_turns: i32) -> f64 {
    let n_turns_float = n_turns as f64;
    let mut expected_crosses = 0.0;
    for i in (1..(n_turns + 1)) {
        let conv_prob = calc_conv_prob_combinations(i, prob_down);
        println!("{} : {}", i, conv_prob);
        expected_crosses += conv_prob;
    }
    return expected_crosses;
}

fn calc_conv_prob_combinations(n: i32, prob_down: f64) -> f64 {
    let n_combinations = calc_combinations(n);
    let prob_up = 1.0 - prob_down;
    let prob_of_one = prob_up.powi(n) * prob_down.powi(n);
    return n_combinations * prob_of_one;
}

fn calc_combinations(n: i32) -> f64 {
    let n_f64 = n as f64;
    return math_tools::fact(n * 2) / (math_tools::fact(n) * math_tools::fact(n))
}


pub fn calc_loss_combinations(n: i32, prob_down: f64) -> f64 {
    let mut total_loss = 0.0;
    let prob_up = 1.0 - prob_down;
    for i in 0..(n+1) {
        let f = i as f64;
        let combs = math_tools::fact(n * 2) / (math_tools::fact(n+i) * math_tools::fact(n-i));
        // We multiply f by 2 because at these points 
        // losses about even numbers 
        // and we multiyply combs by 2 because of symmetry 
        let mut temp_loss = f * 2.0;
        if temp_loss > 0.0 {
            // We need to subtract 1 because we buy and asset
            // after it reaches point 1 
            temp_loss = temp_loss - 1.0;
        }
        let single_loss = temp_loss * (prob_up.powi(n+i) * prob_down.powi(n-i)) * combs * 2.0;
        total_loss += single_loss;
        }
    return total_loss;
}







