extern crate num;
extern crate rand;

use num::traits::Float;

mod simulation_part;
mod simulation_combs;
mod analytics_combs;

fn main() {

    // The results are intersing,
    // I jsut need to validate them with a simulation  :) 

    let a = 5;   
    let prob_down = 0.5;

    println!("prob to conv at {} is {}", a, calc_conv_prob(a, prob_down));

    // Looks right, but I need validate it
    let n_crosses = analytics_combs::calc_expected_n_crosses_combinations(0.5, 10);
    println!("expected number of crosses: {}", n_crosses);

    let loss = analytics_combs::calc_loss_combinations(10, 0.8);
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


fn calc_conv_prob(n: i32, prob_down: f64) -> f64 {
    let catalan = calc_catalan((n-1));
    let prob_up = 1.0 - prob_down;
    let prob_of_one = prob_up.powi(n-1) * prob_down.powi(n);
    return catalan * prob_of_one;
}

fn calc_catalan(n: i32) -> f64 {
    let n_f64 = n as f64;
    return fact(n * 2) / (fact(n) * fact(n) * ((n_f64+1.0)));
}


pub fn fact(n: i32) -> f64 {
    
    let mut prod = 1.0;
    for i in (1..(n+1)) {
       let f = i as f64; 
       prod = prod * f;
    }
        
   return prod;
}

