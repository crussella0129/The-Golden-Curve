fn main() {
    println!("The Golden Curve");
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
}
