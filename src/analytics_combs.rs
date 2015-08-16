use math_tools;
use std::collections::HashMap;

pub fn calc_expected_n_crosses_combinations(prob_down: f64, n_turns: i32) -> f64 {
    let n_turns_float = n_turns as f64;
    let mut expected_crosses = 0.0;
    for i in (1..(n_turns + 1)) {
        let conv_prob = calc_conv_prob_combinations(i, prob_down);
        expected_crosses += conv_prob;
    }
    return expected_crosses;
}

pub fn make_break_map(n_before_break: i32, prob_down: f64) -> HashMap<i32, f64> {
    let mut my_map = HashMap::<i32, f64>::new();
         
    for i in 0..(n_before_break + 1) {
        let p = calc_conv_prob_combinations_break(n_before_break, prob_down, i * 2);
        if i == 0 {
            my_map.insert(i, p); 
            } else {
            my_map.insert(i, p * 2.0); // Probability doubles due to symmetry
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

pub fn calc_expected_crosses_after_break(n_before_break: i32, n_after_break: i32, prob_down_before: f64, prob_down_after: f64) -> f64 {
    let break_map = make_break_map(n_before_break, prob_down_before);
    let away_from_cross_to_prob = calc_away_from_cross_to_prob(n_before_break, prob_down_after, n_after_break);
 
    let mut expected_crosses = 0.0;

    println!("break map : {:?}", break_map);
    println!("cross_to_prob : {:?}", away_from_cross_to_prob);

    for i in 0..n_before_break {
        expected_crosses = expected_crosses + away_from_cross_to_prob[&i] * break_map[&i];
        println!("expected_crosses : {} : {}", i, expected_crosses)
    }

    expected_crosses
}



pub fn calc_away_from_cross_to_prob(n_to_break: i32, prob_down: f64, n_from_break_to_end: i32) -> HashMap<i32, f64> {
    // n_from_break should be equal to n_from_cross 
    let mut my_map = HashMap::<i32, f64>::new();

    for i in 0..(n_to_break+1) {
        my_map.insert(i, calc_after_break_exp_crosses_one_point(i, prob_down, n_from_break_to_end));
    }

    my_map
}

// Something is Wrong here
pub fn calc_after_break_exp_crosses(from_cross: i32, prob_down: f64, i: i32) ->  f64 {
    let n_combs = math_tools::fact(i * 2) / (math_tools::fact(i + from_cross) * math_tools::fact(i - from_cross));

    println!("i: {}, n_comb: {}", i, n_combs);

    let prob_up = 1.0 - prob_down;
    let prob_of_one = prob_down.powi(i + from_cross) * prob_up.powi(i - from_cross);

    println!("i: {}, prob_of_one: {}", i, prob_of_one);

    n_combs as f64 * prob_of_one
}

// Something is wrong here
pub fn calc_after_break_exp_crosses_one_point(from_cross: i32, prob_down: f64, n_full_turns_after_break: i32) -> f64 {
    let mut n_expected_crosses = 0.0;
    for i in 1..(n_full_turns_after_break - from_cross + 1) {
        n_expected_crosses = n_expected_crosses + calc_after_break_exp_crosses(from_cross, prob_down, i);
    }
    n_expected_crosses
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







