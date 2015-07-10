extern crate num;
extern crate rand;

use num::traits::Float;

mod simulation_part;
mod simulation_combs;

fn main() {

    // The results are intersing,
    // I jsut need to validate them with a simulation  :) 

    let a = 5;   
    let prob_down = 0.5;

    println!("prob to conv at {} is {}", a, calc_conv_prob(a, prob_down));

    let temp = calc_conv_prob_combinations(3, 0.8);
    println!("prob to conv at 3 is {}", temp);

    // Looks right, but I need validate it
    let n_crosses = calc_expected_n_crosses_combinations(0.5, 10);
    println!("expected number of crosses: {}", n_crosses);

    let loss = calc_loss_combinations(10, 0.8);
    println!("total loss: {}", loss);

    println!("Simulation Result");
    let sim_loss = simulation_combs::many_simulations(10, 0.8, 1000000);
    println!("total loss: {}", sim_loss);
}

fn calc_expected_n_crosses(prob_down: f64, n_turns: i32) -> f64 {
    // This approach seems flawd
    // it way underestimates paths that just drift off
    let n_turns_float = n_turns as f64;
    let mut expected_crosses = 0.0;
    for i in (1..(n_turns + 1)) {
        let conv_prob = calc_conv_prob(i, prob_down);
        println!("{} : {}", i, conv_prob);
        expected_crosses += conv_prob;
    }
    return expected_crosses;
}

fn calc_expected_n_crosses_combinations(prob_down: f64, n_turns: i32) -> f64 {
    // This approach seems flawd
    // it way underestimates paths that just drift off
    let n_turns_float = n_turns as f64;
    let mut expected_crosses = 0.0;
    for i in (1..(n_turns + 1)) {
        let conv_prob = calc_conv_prob_combinations(i, prob_down);
        println!("{} : {}", i, conv_prob);
        expected_crosses += conv_prob;
    }
    return expected_crosses;
}


fn calc_conv_prob(n: i32, prob_down: f64) -> f64 {
    let catalan = calc_catalan((n-1));
    let prob_up = 1.0 - prob_down;
    let prob_of_one = prob_up.powi(n-1) * prob_down.powi(n);
    return catalan * prob_of_one;
}

fn calc_conv_prob_combinations(n: i32, prob_down: f64) -> f64 {
    let n_combinations = calc_combinations(n);
    let prob_up = 1.0 - prob_down;
    let prob_of_one = prob_up.powi(n) * prob_down.powi(n);
    return n_combinations * prob_of_one;
}


fn calc_catalan(n: i32) -> f64 {
    let n_f64 = n as f64;
    return fact(n * 2) / (fact(n) * fact(n) * ((n_f64+1.0)));
}

fn calc_combinations(n: i32) -> f64 {
    let n_f64 = n as f64;
    return fact(n * 2) / (fact(n) * fact(n))
}

// This seems correct but needs more validation
// since losses are greater than profits for prob_down = 0.5 
fn calc_loss_combinations(n: i32, prob_down: f64) -> f64 {
    let mut total_loss = 0.0;
    let prob_up = 1.0 - prob_down;
    for i in 0..(n+1) {
        let f = i as f64;
        let combs = fact(n * 2) / (fact(n+i) * fact(n-i));
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

fn fact(n: i32) -> f64 {
    
    let mut prod = 1.0;
    for i in (1..(n+1)) {
       let f = i as f64; 
       prod = prod * f;
    }
        
   return prod;
}

