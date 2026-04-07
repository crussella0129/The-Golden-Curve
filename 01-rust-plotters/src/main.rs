fn main() {
    println!("{:<6} {:<20}", "n", "x");
    println!("{}", "-".repeat(28));
    for n in [2.0, 3.0, 4.0, 5.0, 10.0, 20.0] {
        println!("{:<6} {:.16}", n, newton(n));
    }
}

fn newton(n: f64) -> f64 {
    let mut x = 1.5_f64;
    for _ in 0..100 {
        let fx = x.powf(n) - x - 1.0;
        let dfx = n * x.powf(n - 1.0) - 1.0;
        let x_new = x - fx / dfx;
        if (x_new - x).abs() < 1e-12 {
            return x_new;
        }
        x = x_new;
    }
    x
}

fn solve_for_n(x: f64) -> f64 {
    (x + 1.0).ln() / x.ln()
}

/// Sweeps x from x_min to x_max, computes n = ln(x+1)/ln(x) for each point.
/// Returns Vec<(n, x)> sorted by n ascending (ready to plot with n on x-axis).
fn generate_curve(x_min: f64, x_max: f64, steps: usize) -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64)> = (0..steps)
        .map(|i| {
            let x = x_min + (x_max - x_min) * i as f64 / (steps as f64 - 1.0);
            let n = solve_for_n(x);
            (n, x)
        })
        .collect();
    // x increases left to right but n decreases, so sort to get n ascending
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHI: f64 = 1.6180339887498948;
    const PLASTIC: f64 = 1.3247179572447460;

    #[test]
    fn test_newton_n2_gives_phi() {
        let result = newton(2.0);
        assert!(
            (result - PHI).abs() < 1e-10,
            "Expected phi={PHI}, got {result}"
        );
    }

    #[test]
    fn test_newton_n3_gives_plastic() {
        let result = newton(3.0);
        assert!(
            (result - PLASTIC).abs() < 1e-6,
            "Expected P={PLASTIC}, got {result}"
        );
    }

    #[test]
    fn test_solve_for_n_phi_gives_2() {
        let result = solve_for_n(PHI);
        assert!(
            (result - 2.0).abs() < 1e-10,
            "Expected n=2, got {result}"
        );
    }

    #[test]
    fn test_solve_for_n_plastic_gives_3() {
        let result = solve_for_n(PLASTIC);
        assert!(
            (result - 3.0).abs() < 1e-6,
            "Expected n=3, got {result}"
        );
    }

    #[test]
    fn test_generate_curve() {
        let points = generate_curve(1.001, 1.617, 100);
        assert_eq!(points.len(), 100);
        assert!(points.iter().all(|(n, _)| *n >= 1.9), "Some n values below 2");
        assert!(points.iter().all(|(_, x)| *x > 1.0 && *x < 1.62), "x out of range");
        for i in 1..points.len() {
            assert!(points[i].0 >= points[i - 1].0, "Not sorted at index {i}");
        }
    }
}
