mod solver;
mod plotter;

use solver::{newton, generate_curve};
use plotter::plot_curve;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Mode: solve for x given n
    if let Some(pos) = args.iter().position(|a| a == "--n") {
        if let Some(n_str) = args.get(pos + 1) {
            let n: f64 = n_str.parse().expect("--n requires a number");
            println!("x = {:.16}", newton(n));
            return;
        }
    }

    // Mode: solve for n given x
    if let Some(pos) = args.iter().position(|a| a == "--x") {
        if let Some(x_str) = args.get(pos + 1) {
            let x: f64 = x_str.parse().expect("--x requires a number");
            if x <= 1.0 {
                eprintln!("Error: x must be > 1");
                std::process::exit(1);
            }
            println!("n = {:.16}", solver::solve_for_n(x));
            return;
        }
    }

    // Default: print table and generate plot
    println!("{:<6} {:<20}", "n", "x");
    println!("{}", "-".repeat(28));
    for n in [2.0, 3.0, 4.0, 5.0, 10.0, 20.0] {
        println!("{:<6} {:.16}", n, newton(n));
    }
    let points = generate_curve(1.001, 1.617, 800);
    plot_curve(&points).expect("Plotting failed");
}
