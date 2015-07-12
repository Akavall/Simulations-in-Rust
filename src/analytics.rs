use math_tools;

pub fn calc_expected_n_crosses(prob_down: f64, n_turns: i32) -> f64 {
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
    return math_tools::fact(n * 2) / (math_tools::fact(n) * math_tools::fact(n) * ((n_f64+1.0)));
}
