use pyo3::prelude::*;

fn main() {
    // Initialize Python before acquiring the GIL
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        // import_bound() is the PyO3 0.21+ "bound" API
        let sys = py.import_bound("sys").expect("Could not import sys");
        let version: String = sys
            .getattr("version")
            .expect("No version attr")
            .extract()
            .expect("Could not read version string");
        println!("Python version: {version}");
    });
}

fn newton(n: f64) -> f64 {
    let mut x = 1.5_f64;
    for _ in 0..100 {
        let fx = x.powf(n) - x - 1.0;
        let dfx = n * x.powf(n - 1.0) - 1.0;
        let x_new = x - fx / dfx;
        if (x_new - x).abs() < 1e-12 { return x_new; }
        x = x_new;
    }
    x
}

fn solve_for_n(x: f64) -> f64 {
    (x + 1.0).ln() / x.ln()
}

fn generate_curve(x_min: f64, x_max: f64, steps: usize) -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64)> = (0..steps)
        .map(|i| {
            let x = x_min + (x_max - x_min) * i as f64 / (steps as f64 - 1.0);
            (solve_for_n(x), x)
        })
        .collect();
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHI: f64 = 1.6180339887498948;
    const PLASTIC: f64 = 1.3247179572447460;

    #[test]
    fn test_newton_n2() {
        assert!((newton(2.0) - PHI).abs() < 1e-10);
    }

    #[test]
    fn test_newton_n3() {
        assert!((newton(3.0) - PLASTIC).abs() < 1e-6);
    }

    #[test]
    fn test_solve_for_n() {
        assert!((solve_for_n(PHI) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_curve_sorted() {
        let pts = generate_curve(1.001, 1.617, 50);
        assert_eq!(pts.len(), 50);
        for i in 1..pts.len() {
            assert!(pts[i].0 >= pts[i-1].0);
        }
    }
}
