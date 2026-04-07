use plotters::prelude::*;

fn main() {
    println!("{:<6} {:<20}", "n", "x");
    println!("{}", "-".repeat(28));
    for n in [2.0, 3.0, 4.0, 5.0, 10.0, 20.0] {
        println!("{:<6} {:.16}", n, newton(n));
    }

    let points = generate_curve(1.001, 1.617, 800);
    plot_curve(&points).expect("Plotting failed");
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
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    points
}

fn plot_curve(points: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    const PHI: f64 = 1.6180339887498948;
    const N_MIN: f64 = 2.0;
    const N_MAX: f64 = 20.0;
    const X_MIN: f64 = 1.0;
    const X_MAX: f64 = PHI + 0.08;

    let root = BitMapBackend::new("golden_curve.png", (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("The Golden Curve: x^n = x + 1", ("sans-serif", 22))
        .margin(40)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(N_MIN..N_MAX, X_MIN..X_MAX)?;

    chart
        .configure_mesh()
        .x_desc("n  (exponent)")
        .y_desc("x  (solution)")
        .draw()?;

    // The curve -- filter to display range
    let curve: Vec<(f64, f64)> = points
        .iter()
        .filter(|(n, _)| *n >= N_MIN && *n <= N_MAX)
        .copied()
        .collect();
    chart.draw_series(LineSeries::new(curve, BLUE.mix(0.8)))?;

    // Dashed asymptote at x = 1
    chart.draw_series(DashedLineSeries::new(
        vec![(N_MIN, 1.0), (N_MAX, 1.0)],
        5,
        8,
        ShapeStyle {
            color: BLACK.mix(0.3),
            filled: false,
            stroke_width: 1,
        },
    ))?;

    // Dot and label at phi (n=2)
    chart.draw_series(PointSeries::of_element(
        vec![(2.0_f64, PHI)],
        7,
        RED.filled(),
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st),
    ))?;
    chart.draw_series(std::iter::once(Text::new(
        "phi ~= 1.618",
        (2.2, PHI - 0.04),
        ("sans-serif", 14).into_font().color(&RED),
    )))?;

    // Dot and label at plastic constant (n=3)
    let plastic = newton(3.0);
    chart.draw_series(PointSeries::of_element(
        vec![(3.0_f64, plastic)],
        7,
        GREEN.mix(0.8).filled(),
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st),
    ))?;
    chart.draw_series(std::iter::once(Text::new(
        "P ~= 1.325",
        (3.2, plastic - 0.04),
        ("sans-serif", 14).into_font().color(&GREEN),
    )))?;

    root.present()?;
    println!("Saved: golden_curve.png");
    Ok(())
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
