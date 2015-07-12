extern crate num;
extern crate rand;

use num::traits::Float;

mod math_tools;
mod simulation_part;
mod simulation_combs;
mod analytics;
mod analytics_combs;
mod simulation_combs_w_break;

fn main() {

    let temp = analytics::calc_expected_n_crosses(0.8, 10);
    println!("temp : {}", temp);

    let n_crosses = analytics_combs::calc_expected_n_crosses_combinations(0.5, 10);
    println!("expected number of crosses: {}", n_crosses);

    let loss = analytics_combs::calc_loss_combinations(10, 0.5);
    println!("total loss: {}", loss);

    println!("Simulation Result");
    let sim_loss = simulation_combs::many_simulations(10, 0.5, 1000000);
    println!("total loss: {}", sim_loss);

    let sim_loss_wb = simulation_combs_w_break::many_simulations_losses(5, 5, 0.5, 0.5, 1000000);
    println!("total loss w b: {}", sim_loss_wb);

    let sim_cross_wb = simulation_combs_w_break::many_simulations_crosses(5, 5, 0.5, 0.5, 1000000);
    println!("total cross w b: {}", sim_cross_wb);



}


