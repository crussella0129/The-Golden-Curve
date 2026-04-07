use crate::solver::newton;
use plotters::prelude::*;

pub fn plot_curve(points: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
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

    let curve: Vec<(f64, f64)> = points
        .iter()
        .filter(|(n, _)| *n >= N_MIN && *n <= N_MAX)
        .copied()
        .collect();
    chart.draw_series(LineSeries::new(curve, BLUE.mix(0.8)))?;

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
