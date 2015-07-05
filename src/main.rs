extern crate num;
extern crate rand;

use num::traits::Float;

mod simulation_part;

fn main() {

    // The results are intersing,
    // I jsut need to validate them with a simulation  :) 

    let a = 5;   
    let prob_down = 0.5;

    println!("prob to conv at {} is {}", a, calc_conv_prob(a, prob_down));
    /*
    let mut cum_sum = 0.0;
    for i in (1..10) {
        let prob = calc_conv_prob(i, prob_up, prob_down);
        cum_sum += prob;
        println!("turn : {}, prob : {}, cum_sum : {}", i, prob, cum_sum);
    }

    let my_counter = simulation_part::gen_conv_dist(0.5, 10, 1000000);
    println!("counter : {:?}", my_counter);
    */

    let n_covs = simulation_part::gen_n_convs_one_run(0.5, 10);
    println!("n_covs : {}", n_covs);

    let expected_crosses = calc_expected_n_crosses(0.5, 10);
    println!("Expected crosses : {}", expected_crosses);

    let sim_crosses = simulation_part::gen_expected_crosses(0.5, 10, 1000000);
    println!("Expected crosses sim : {}", sim_crosses);

    // Something is way off, there is a big difference :( 
    
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

fn fact(n: i32) -> f64 {
    
    let mut prod = 1.0;
    for i in (1..(n+1)) {
       let f = i as f64; 
       prod = prod * f;
    }
        
   return prod;
}

