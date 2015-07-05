extern crate rand;

use rand::Rng;
use std::collections::HashMap;

pub fn gen_conv_dist(prob_down: f64, n_turns: i32, n_iterations: i32) -> HashMap<i32, i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for _ in (0..n_iterations) {
        let turns = gen_conv_one_run(prob_down, n_turns);
        if counter.contains_key(&turns) {
            *counter.get_mut(&turns).unwrap() += 1;
        } else {
            counter.insert(turns, 1);
        }
    }
    return counter;
}

pub fn gen_conv_one_run(prob_down: f64, n_turns: i32) -> i32 {
    let mut up_down_balance = 1;
    for i in (2..(n_turns * 2)) {
        let roll = rand::thread_rng().gen_range(0.0, 1.0);
        if roll < prob_down {
            up_down_balance -= 1;
        } else {
            up_down_balance += 1;
        }
        if up_down_balance == 0 {
            return i / 2;
        }
    }
    return -1; 
}

pub fn gen_expected_crosses(prob_down: f64, n_turns: i32, n_iterations: i32) -> f64 {
    let mut n_crosses = 0;
    for _ in (0..n_iterations) {
        n_crosses += gen_n_convs_one_run(prob_down, n_turns);
    }
    return (n_crosses as f64) / (n_iterations as f64);
}

pub fn gen_n_convs_one_run(prob_down: f64, n_turns: i32) -> i32 {
    let mut n_convs = 0;
    let mut up_down_balance = 0;
    let mut semi_turns_taken = 0; 

    loop {
        if up_down_balance == 0 {
            up_down_balance += 1;
            semi_turns_taken += 1 
        }

        let roll = rand::thread_rng().gen_range(0.0, 1.0);
        if roll < prob_down {
            up_down_balance -= 1;
        } else {
            up_down_balance += 1;
        }
        if up_down_balance == 0 {
            n_convs += 1;
        }
        semi_turns_taken += 1;

        if semi_turns_taken >= n_turns * 2 {
            return n_convs;
        }
    }
}
