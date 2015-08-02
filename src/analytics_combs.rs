use math_tools;
use std::collections::HashMap;

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

pub fn make_break_map(n_before_break: i32, prob_down: f64) -> HashMap<i32, f64> {
    let mut my_map = HashMap::<i32, f64>::new();
    let n_full_turns = n_before_break / 2; // n_before_break is measured in half_turns
                                      
    for i in 0..(n_full_turns + 1) {
        let p = calc_conv_prob_combinations_break(n_full_turns, prob_down, i * 2);
        if i == 0 {
            my_map.insert(i * 2, p); 
            } else  {
            my_map.insert(i * 2, p * 2.0); // Probability doubles due to symmetry
        }
    }
    my_map
}

fn calc_conv_prob_combinations(n: i32, prob_down: f64) -> f64 {
    let n_combinations = calc_combinations(n);
    let prob_up = 1.0 - prob_down;
    let prob_of_one = prob_up.powi(n) * prob_down.powi(n);
    return n_combinations * prob_of_one; 
}

fn calc_conv_prob_combinations_break(n: i32, prob_down: f64, from_cross: i32) -> f64 {
    // If prob_down = 1.0; dict with have all values as zeros
    // desired behavior, we never diverge 
    let n_combinations = calc_combinations_break(n, from_cross);
    let prob_up = 1.0 - prob_down;
    let a = from_cross / 2;
    let prob_of_one = prob_up.powi(n + a) * prob_down.powi(n - a);
    return n_combinations * prob_of_one; 
}

fn calc_combinations(n: i32) -> f64 {
    let n_f64 = n as f64;
    return math_tools::fact(n * 2) / (math_tools::fact(n) * math_tools::fact(n))
}

fn calc_combinations_break(n: i32, from_cross: i32) -> f64 {
    let n_f64 = n as f64;
    let a = from_cross / 2;
    return math_tools::fact(n * 2) / (math_tools::fact(n + a) * math_tools::fact(n - a))
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







