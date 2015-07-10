extern crate rand;

use rand::Rng;

pub fn many_simulations(n: i32, prob_down: f64, n_simulations: i32) -> f64 {
    let mut sum_loss = 0;
    for i in 0..n_simulations {
        sum_loss += one_round(n, prob_down);
    }


    return (sum_loss as f64) / (n_simulations as f64);
}


fn one_round(n: i32, prob_down: f64) -> i32 {
    let mut start = 0;
    for i in 0..(n * 2) {
        let roll = rand::thread_rng().gen_range(0.0, 1.0);
        if roll > prob_down {
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
