mod solver;
mod plotter;

use solver::{newton, generate_curve};
use plotter::plot_curve;

fn main() {
    println!("{:<6} {:<20}", "n", "x");
    println!("{}", "-".repeat(28));
    for n in [2.0, 3.0, 4.0, 5.0, 10.0, 20.0] {
        println!("{:<6} {:.16}", n, newton(n));
    }

    let points = generate_curve(1.001, 1.617, 800);
    plot_curve(&points).expect("Plotting failed");
}
