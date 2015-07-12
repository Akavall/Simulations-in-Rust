extern crate rand;

use rand::Rng;

pub fn many_simulations(n_1: i32, n_2: i32, prob_down_1: f64, prob_down_2: f64, n_simulations: i32) -> f64 {
    let mut sum_loss = 0;
    for i in 0..n_simulations {
        sum_loss += one_round(n_1, n_2, prob_down_1, prob_down_2);
    }
    
    return (sum_loss as f64) / (n_simulations as f64);
}

fn one_round(n_1: i32, n_2: i32, prob_down_1: f64, prob_down_2: f64) -> i32 {
    let mut start = 0;
    for i in 0..(n_1 * 2) {
        let roll = rand::thread_rng().gen_range(0.0, 1.0);
        if roll > prob_down_1 {
            start += 1;
        } else {
            start -= 1;
        }
    }
    for i in 0..(n_2 * 2) {
        let roll = rand::thread_rng().gen_range(0.0, 1.0);
        if roll > prob_down_2 {
            start += 1;
        } else {
            start -= 1;
        }
    }
    if start > 0 {
        return (start - 1) * 2;
    }
    return 0; 
}
