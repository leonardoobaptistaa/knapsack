//my first rust code \o/

fn main() {
    let gallon_size: f64 = 5.0;
    let real_bottles: [f64; 4] = [1.0, 3.0, 3.5, 1.5];

    let bottles: Vec<f64> = (0..4*60)
        .map(|n| real_bottles[n % 4])
        .collect();

    let result = knapsack(gallon_size, &bottles, bottles.len());
    println!("{}", result);
}

fn knapsack(cap:f64, weights: &[f64], n:usize) -> f64 {
    if n == 0 || cap == 0.0 {
        return 0.0;
    }

    let evaluating = *(weights.get(n - 1).unwrap()) as f64;
    if evaluating > cap {
        return knapsack(cap, weights, n - 1);
    }

    let prev_result = evaluating + knapsack(cap - evaluating, weights, n - 1);
    let result = knapsack(cap, weights, n - 1);
    return prev_result.max(result);
}

