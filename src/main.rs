extern crate num;
extern crate rand;

use num::traits::Float;

mod math_tools;
mod simulation_part;
mod simulation_combs;
mod analytics;
mod analytics_combs;

fn main() {

    let temp = analytics::calc_expected_n_crosses(0.8, 10);
    println!("temp : {}", temp);

    let n_crosses = analytics_combs::calc_expected_n_crosses_combinations(0.5, 10);
    println!("expected number of crosses: {}", n_crosses);

    let loss = analytics_combs::calc_loss_combinations(10, 0.8);
    println!("total loss: {}", loss);

    println!("Simulation Result");
    let sim_loss = simulation_combs::many_simulations(10, 0.8, 100000);
    println!("total loss: {}", sim_loss);
}


