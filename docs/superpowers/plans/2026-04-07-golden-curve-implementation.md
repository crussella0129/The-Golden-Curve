# The Golden Curve — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rosetta Stone repo — the equation `x^n = x+1` solved and visualized three ways: pure Rust, pure Python, and Rust+PyO3+Manim.

**Architecture:** Three independent lesson folders, each self-contained with its own dependencies and entry point. Lessons share the same math (Newton's method + closed-form `n = ln(x+1)/ln(x)`) but use entirely different toolchains. TDD throughout — write the test, watch it fail, implement, watch it pass.

**Tech Stack:** Rust 2021 + `plotters 0.3` (Lesson 1) · Python 3.11+ + `manim` (Lesson 2) · Rust 2021 + `pyo3 0.22` + `manim` (Lesson 3)

---

## Verified math reference (consult when stuck)

```
f(x)  = x^n - x - 1
f'(x) = n · x^(n-1) - 1
Newton: x_new = x - f(x)/f'(x),  x₀ = 1.5,  stop when |Δx| < 1e-12

Closed form (given x): n = ln(x+1) / ln(x)   [x > 1 only]

Known values (use as test targets):
  φ     = 1.6180339887498948   (n=2)
  P     = 1.3247179572447460   (n=3, plastic constant)
  x(10) ≈ 1.0718               (n=10, approx)
  x(20) ≈ 1.0353               (n=20, approx)
```

---

## File Map

```
The-Golden-Curve/
├── docs/                          (already exists)
│
├── 01-rust-plotters/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                (all code starts here)
│       ├── solver.rs              (introduced in Task 5)
│       └── plotter.rs             (introduced in Task 5)
│
├── 02-python-manim/
│   ├── requirements.txt
│   ├── solver.py
│   ├── scene.py
│   └── tests/
│       └── test_solver.py
│
└── 03-rust-pyo3-manim/
    ├── Cargo.toml
    ├── requirements.txt
    ├── render.py
    └── src/
        └── main.rs
```

---

# Lesson 1 — Pure Rust + plotters

---

### Task 1: Scaffold the Rust project

**Files:**
- Create: `01-rust-plotters/Cargo.toml`
- Create: `01-rust-plotters/src/main.rs`

- [ ] **Step 1: Create the project**

```bash
cd C:/Users/charl/The-Golden-Curve
cargo new --name golden_curve 01-rust-plotters
```

Expected output: `Created binary (application) \`golden_curve\` package`

- [ ] **Step 2: Replace Cargo.toml**

```toml
[package]
name = "golden_curve"
version = "0.1.0"
edition = "2021"

[dependencies]
plotters = "0.3"
```

- [ ] **Step 3: Verify it compiles**

```bash
cd 01-rust-plotters
cargo build
```

Expected: `Compiling golden_curve v0.1.0` then `Finished`. If plotters fails to download, check your internet connection.

- [ ] **Step 4: Commit**

```bash
git add 01-rust-plotters/
git commit -m "feat: scaffold Lesson 1 - Rust+plotters"
```

---

### Task 2: Newton's method and closed-form solver

**Files:**
- Modify: `01-rust-plotters/src/main.rs`

- [ ] **Step 1: Write the failing tests**

Replace the contents of `src/main.rs` with:

```rust
fn main() {
    println!("The Golden Curve");
}

fn newton(n: f64) -> f64 {
    todo!("implement Newton's method")
}

fn solve_for_n(x: f64) -> f64 {
    todo!("implement closed form: ln(x+1)/ln(x)")
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
            "Expected φ≈{PHI}, got {result}"
        );
    }

    #[test]
    fn test_newton_n3_gives_plastic() {
        let result = newton(3.0);
        assert!(
            (result - PLASTIC).abs() < 1e-6,
            "Expected P≈{PLASTIC}, got {result}"
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
```

- [ ] **Step 2: Run tests — confirm they fail with `todo!()`**

```bash
cargo test 2>&1 | head -30
```

Expected: `test tests::test_newton_n2_gives_phi ... FAILED` (panicked at 'not yet implemented')

- [ ] **Step 3: Implement `newton`**

```rust
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
    x  // return best estimate if max iterations reached
}
```

- [ ] **Step 4: Implement `solve_for_n`**

```rust
fn solve_for_n(x: f64) -> f64 {
    (x + 1.0).ln() / x.ln()
}
```

- [ ] **Step 5: Run tests — confirm they pass**

```bash
cargo test
```

Expected:
```
test tests::test_newton_n2_gives_phi     ... ok
test tests::test_newton_n3_gives_plastic ... ok
test tests::test_solve_for_n_phi_gives_2 ... ok
test tests::test_solve_for_n_plastic_gives_3 ... ok

test result: ok. 4 passed; 0 failed
```

- [ ] **Step 6: Commit**

```bash
git add src/main.rs
git commit -m "feat(L1): implement Newton solver and closed-form n(x)"
```

---

### Task 3: Curve generation

**Files:**
- Modify: `01-rust-plotters/src/main.rs`

- [ ] **Step 1: Write the failing test**

Add inside `mod tests {}`:

```rust
    #[test]
    fn test_generate_curve() {
        let points = generate_curve(1.001, 1.617, 100);
        assert_eq!(points.len(), 100);

        // n values should all be >= 2 (x <= φ means n >= 2)
        assert!(points.iter().all(|(n, _)| *n >= 1.9),
            "Some n values below 2");

        // x values should stay in (1, φ+margin)
        assert!(points.iter().all(|(_, x)| *x > 1.0 && *x < 1.62),
            "x values out of expected range");

        // Should be sorted by n ascending
        for i in 1..points.len() {
            assert!(points[i].0 >= points[i - 1].0,
                "Points not sorted by n at index {i}");
        }
    }
```

- [ ] **Step 2: Run to confirm failure**

```bash
cargo test test_generate_curve
```

Expected: `error[E0425]: cannot find function \`generate_curve\``

- [ ] **Step 3: Implement `generate_curve`**

Add after `solve_for_n`:

```rust
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
```

- [ ] **Step 4: Run tests**

```bash
cargo test
```

Expected: `test result: ok. 5 passed; 0 failed`

- [ ] **Step 5: Print a verification table in main**

Replace `fn main()`:

```rust
fn main() {
    println!("{:<6} {:<20}", "n", "x");
    println!("{}", "-".repeat(28));
    for n in [2.0, 3.0, 4.0, 5.0, 10.0, 20.0] {
        println!("{:<6} {:.16}", n, newton(n));
    }
}
```

Run: `cargo run`

Expected output:
```
n      x
----------------------------
2      1.6180339887498949
3      1.3247179572447460
4      1.2207440846057595
5      1.1673039782614187
10     1.0718...
20     1.0353...
```

- [ ] **Step 6: Commit**

```bash
git add src/main.rs
git commit -m "feat(L1): add curve generation + verification table"
```

---

### Task 4: plotters visualization

**Files:**
- Modify: `01-rust-plotters/src/main.rs`

- [ ] **Step 1: Add the plot function (no test — visual output)**

Add after `generate_curve`:

```rust
use plotters::prelude::*;

fn plot_curve(points: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    const PHI: f64 = 1.6180339887498948;
    const N_MIN: f64 = 2.0;
    const N_MAX: f64 = 20.0;
    const X_MIN: f64 = 1.0;
    const X_MAX: f64 = PHI + 0.08;

    let root = BitMapBackend::new("golden_curve.png", (900, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("The Golden Curve:  xn = x + 1", ("sans-serif", 22))
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
    chart.draw_series(LineSeries::new(curve, &BLUE.mix(0.8)))?;

    // Dashed asymptote at x = 1
    chart.draw_series(DashedLineSeries::new(
        vec![(N_MIN, 1.0), (N_MAX, 1.0)],
        5, 5, Into::<ShapeStyle>::into(&BLACK.mix(0.3)),
    ))?;

    // Dot at phi (n=2)
    chart.draw_series(PointSeries::of_element(
        vec![(2.0_f64, PHI)],
        7,
        &RED,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    chart.draw_series(std::iter::once(Text::new(
        "phi ~= 1.618",
        (2.2, PHI - 0.04),
        ("sans-serif", 14).into_font().color(&RED),
    )))?;

    // Dot at plastic constant (n=3)
    let plastic = newton(3.0);
    chart.draw_series(PointSeries::of_element(
        vec![(3.0_f64, plastic)],
        7,
        &GREEN.mix(0.8),
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    chart.draw_series(std::iter::once(Text::new(
        "P ~= 1.325",
        (3.2, plastic - 0.04),
        ("sans-serif", 14).into_font().color(&GREEN.mix(0.8).to_rgba()),
    )))?;

    root.present()?;
    println!("Saved: golden_curve.png");
    Ok(())
}
```

- [ ] **Step 2: Call it from main**

```rust
fn main() {
    // Verification table
    println!("{:<6} {:<20}", "n", "x");
    println!("{}", "-".repeat(28));
    for n in [2.0, 3.0, 4.0, 5.0, 10.0, 20.0] {
        println!("{:<6} {:.16}", n, newton(n));
    }

    // Generate and plot
    let points = generate_curve(1.001, 1.617, 800);
    plot_curve(&points).expect("Plotting failed");
}
```

- [ ] **Step 3: Build and run**

```bash
cargo run
```

Expected: table printed, then `Saved: golden_curve.png`

- [ ] **Step 4: Open and inspect the PNG**

Open `01-rust-plotters/golden_curve.png`. Verify:
- Curve starts at top-left (~2, 1.618) and curves down-right toward (20, ~1.035)
- Red dot labeled phi at (2, 1.618)
- Green dot labeled P at (3, 1.325)
- Dashed line near x=1 (the asymptote)

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat(L1): add plotters visualization, golden_curve.png"
```

---

### Task 5: Module refactor — B-level concept

> **B-level concept:** In Rust, `mod` splits code into separate files. The compiler finds `solver.rs` automatically when you write `mod solver;` in `main.rs`. Each module has its own privacy boundary — `pub fn` exports, private by default.

**Files:**
- Create: `01-rust-plotters/src/solver.rs`
- Create: `01-rust-plotters/src/plotter.rs`
- Modify: `01-rust-plotters/src/main.rs`

- [ ] **Step 1: Create `src/solver.rs`**

Move `newton`, `solve_for_n`, and `generate_curve` here. Add `pub` to each:

```rust
/// Newton's method: given n, find x such that x^n = x + 1.
pub fn newton(n: f64) -> f64 {
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

/// Closed form: given x, return n = ln(x+1)/ln(x).
pub fn solve_for_n(x: f64) -> f64 {
    (x + 1.0).ln() / x.ln()
}

/// Sweep x in [x_min, x_max], compute n for each, return (n, x) sorted by n ascending.
pub fn generate_curve(x_min: f64, x_max: f64, steps: usize) -> Vec<(f64, f64)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    const PHI: f64 = 1.6180339887498948;
    const PLASTIC: f64 = 1.3247179572447460;

    #[test]
    fn test_newton_n2_gives_phi() {
        let result = newton(2.0);
        assert!((result - PHI).abs() < 1e-10, "Expected phi={PHI}, got {result}");
    }

    #[test]
    fn test_newton_n3_gives_plastic() {
        let result = newton(3.0);
        assert!((result - PLASTIC).abs() < 1e-6, "Expected P={PLASTIC}, got {result}");
    }

    #[test]
    fn test_solve_for_n_phi_gives_2() {
        let result = solve_for_n(PHI);
        assert!((result - 2.0).abs() < 1e-10, "Expected n=2, got {result}");
    }

    #[test]
    fn test_solve_for_n_plastic_gives_3() {
        let result = solve_for_n(PLASTIC);
        assert!((result - 3.0).abs() < 1e-6, "Expected n=3, got {result}");
    }

    #[test]
    fn test_generate_curve() {
        let points = generate_curve(1.001, 1.617, 100);
        assert_eq!(points.len(), 100);
        assert!(points.iter().all(|(n, _)| *n >= 1.9));
        assert!(points.iter().all(|(_, x)| *x > 1.0 && *x < 1.62));
        for i in 1..points.len() {
            assert!(points[i].0 >= points[i - 1].0);
        }
    }
}
```

- [ ] **Step 2: Create `src/plotter.rs`**

Move `plot_curve` and its imports here:

```rust
use crate::solver::newton;
use plotters::prelude::*;

pub fn plot_curve(points: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    const PHI: f64 = 1.6180339887498948;
    const N_MIN: f64 = 2.0;
    const N_MAX: f64 = 20.0;
    const X_MIN: f64 = 1.0;
    const X_MAX: f64 = PHI + 0.08;

    let root = BitMapBackend::new("golden_curve.png", (900, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("The Golden Curve:  xn = x + 1", ("sans-serif", 22))
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
    chart.draw_series(LineSeries::new(curve, &BLUE.mix(0.8)))?;

    chart.draw_series(DashedLineSeries::new(
        vec![(N_MIN, 1.0), (N_MAX, 1.0)],
        5, 5, Into::<ShapeStyle>::into(&BLACK.mix(0.3)),
    ))?;

    chart.draw_series(PointSeries::of_element(
        vec![(2.0_f64, PHI)],
        7, &RED,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    chart.draw_series(std::iter::once(Text::new(
        "phi ~= 1.618",
        (2.2, PHI - 0.04),
        ("sans-serif", 14).into_font().color(&RED),
    )))?;

    let plastic = newton(3.0);
    chart.draw_series(PointSeries::of_element(
        vec![(3.0_f64, plastic)],
        7, &GREEN.mix(0.8),
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    chart.draw_series(std::iter::once(Text::new(
        "P ~= 1.325",
        (3.2, plastic - 0.04),
        ("sans-serif", 14).into_font().color(&GREEN.mix(0.8).to_rgba()),
    )))?;

    root.present()?;
    println!("Saved: golden_curve.png");
    Ok(())
}
```

- [ ] **Step 3: Replace `src/main.rs`**

```rust
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
```

- [ ] **Step 4: Run tests and binary**

```bash
cargo test && cargo run
```

Expected: 5 tests pass, PNG regenerated.

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "refactor(L1): split into solver.rs and plotter.rs modules"
```

---

### Task 6: Simple CLI — C-level concept

> **C-level concept:** `std::env::args()` gives you `argv` as an iterator. No external crates needed for simple flag parsing. Pattern: collect args, look for `--flag`, grab the next token.

**Files:**
- Modify: `01-rust-plotters/src/main.rs`

- [ ] **Step 1: Replace `fn main()`**

```rust
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
```

- [ ] **Step 2: Test the CLI modes**

```bash
cargo run -- --n 2
# Expected: x = 1.6180339887498949

cargo run -- --n 3
# Expected: x = 1.3247179572447460

cargo run -- --x 1.6180339887498948
# Expected: n = 2.0000000000000000

cargo run -- --x 0.5
# Expected: Error: x must be > 1
```

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat(L1): add CLI flags --n and --x"
```

---

# Lesson 2 — Pure Python + Manim

---

### Task 7: Python solver

**Files:**
- Create: `02-python-manim/requirements.txt`
- Create: `02-python-manim/solver.py`
- Create: `02-python-manim/tests/test_solver.py`

- [ ] **Step 1: Create requirements.txt**

```
manim
pytest
```

- [ ] **Step 2: Install dependencies**

```bash
cd C:/Users/charl/The-Golden-Curve/02-python-manim
pip install -r requirements.txt
```

Manim on Windows may take a few minutes. Verify: `manim --version` should print `Manim Community v0.18.x` (or similar).

- [ ] **Step 3: Write the failing tests**

Create `tests/test_solver.py`:

```python
import math
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from solver import newton, solve_for_n, generate_curve

PHI     = (1 + math.sqrt(5)) / 2          # 1.6180339887498948...
PLASTIC = 1.3247179572447460               # real root of x^3 = x + 1

def test_newton_n2_gives_phi():
    assert abs(newton(2) - PHI) < 1e-10

def test_newton_n3_gives_plastic():
    assert abs(newton(3) - PLASTIC) < 1e-6

def test_solve_for_n_phi_gives_2():
    assert abs(solve_for_n(PHI) - 2.0) < 1e-10

def test_solve_for_n_plastic_gives_3():
    assert abs(solve_for_n(PLASTIC) - 3.0) < 1e-6

def test_generate_curve_length_and_order():
    points = generate_curve(steps=100)
    assert len(points) == 100
    ns = [n for n, _ in points]
    assert all(ns[i] <= ns[i+1] for i in range(len(ns)-1))

def test_generate_curve_bounds():
    points = generate_curve(steps=50)
    ns = [n for n, _ in points]
    xs = [x for _, x in points]
    assert all(n >= 1.9 for n in ns)
    assert all(x > 1.0 for x in xs)
    assert all(x <= PHI + 0.01 for x in xs)
```

- [ ] **Step 4: Run — confirm failures**

```bash
pytest tests/ -v 2>&1 | head -20
```

Expected: `ModuleNotFoundError: No module named 'solver'`

- [ ] **Step 5: Implement `solver.py`**

Create `solver.py`:

```python
import math

PHI     = (1 + math.sqrt(5)) / 2
PLASTIC = 1.3247179572447460

def newton(n: float, x0: float = 1.5, tol: float = 1e-12, max_iter: int = 100) -> float:
    """Given n, find x such that x^n = x + 1 using Newton's method."""
    x = x0
    for _ in range(max_iter):
        fx  = x**n - x - 1
        dfx = n * x**(n - 1) - 1
        x_new = x - fx / dfx
        if abs(x_new - x) < tol:
            return x_new
        x = x_new
    return x

def solve_for_n(x: float) -> float:
    """Given x > 1, return n = ln(x+1)/ln(x). Exact closed form, no iteration."""
    return math.log(x + 1) / math.log(x)

def generate_curve(
    x_min: float = 1.001,
    x_max: float = 1.617,
    steps: int = 500,
) -> list[tuple[float, float]]:
    """
    Sweep x from x_min to x_max, compute n = ln(x+1)/ln(x) for each.
    Returns list of (n, x) tuples sorted by n ascending.
    """
    xs = [x_min + (x_max - x_min) * i / (steps - 1) for i in range(steps)]
    points = [(solve_for_n(x), x) for x in xs]
    return sorted(points)
```

- [ ] **Step 6: Run tests — confirm all pass**

```bash
pytest tests/ -v
```

Expected:
```
tests/test_solver.py::test_newton_n2_gives_phi            PASSED
tests/test_solver.py::test_newton_n3_gives_plastic        PASSED
tests/test_solver.py::test_solve_for_n_phi_gives_2        PASSED
tests/test_solver.py::test_solve_for_n_plastic_gives_3    PASSED
tests/test_solver.py::test_generate_curve_length_and_order PASSED
tests/test_solver.py::test_generate_curve_bounds           PASSED

6 passed in X.XXs
```

- [ ] **Step 7: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 02-python-manim/
git commit -m "feat(L2): implement Python solver with pytest suite"
```

---

### Task 8: Manim scene — skeleton and curve

**Files:**
- Create: `02-python-manim/scene.py`

- [ ] **Step 1: Create scene skeleton**

```python
from manim import *
from solver import newton, generate_curve

PHI     = (1 + 5**0.5) / 2
PLASTIC = newton(3)


class GoldenCurveScene(Scene):
    def construct(self):
        # Axes: n on horizontal (2..20), x on vertical (1..phi+0.1)
        axes = Axes(
            x_range=[2, 20, 2],
            y_range=[1, PHI + 0.1, 0.1],
            x_length=10,
            y_length=5.5,
            axis_config={"include_tip": False},
        )

        x_label = axes.get_x_axis_label(Text("n", font_size=28))
        y_label = axes.get_y_axis_label(Text("x", font_size=28))

        self.add(axes, x_label, y_label)
        self.wait()
```

- [ ] **Step 2: Render the skeleton**

```bash
cd C:/Users/charl/The-Golden-Curve/02-python-manim
manim -s --output_file golden_curve scene.py GoldenCurveScene
```

Expected: Manim renders to `media/images/golden_curve.png`. Open and verify the axes appear.

- [ ] **Step 3: Add the curve**

Replace `self.add(axes, x_label, y_label)` and `self.wait()` with:

```python
        # axes.plot(f) where f(n) -> x uses the "given n, find x" mode
        curve = axes.plot(
            lambda n: newton(n),
            x_range=[2.01, 19.99, 0.05],   # step 0.05 -> 360 sample points
            color=BLUE,
            stroke_width=3,
        )

        asymptote = DashedLine(
            start=axes.c2p(2, 1),
            end=axes.c2p(20, 1),
            dash_length=0.15,
            color=GRAY,
            stroke_width=1.5,
        )

        self.add(axes, x_label, y_label, curve, asymptote)
        self.wait()
```

- [ ] **Step 4: Render and inspect**

```bash
manim -s --output_file golden_curve scene.py GoldenCurveScene
```

Verify: smooth blue curve descending from (2, 1.618) toward (20, ~1.035), dashed gray line near bottom.

- [ ] **Step 5: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 02-python-manim/scene.py
git commit -m "feat(L2): add Manim scene with axes and golden curve"
```

---

### Task 9: Key points, labels, and title

**Files:**
- Modify: `02-python-manim/scene.py`

- [ ] **Step 1: Add labeled key points and title**

Replace `self.add(...)` with:

```python
        phi_dot = Dot(axes.c2p(2, PHI), color=GOLD, radius=0.09)
        phi_label = Text(f"phi ~= {PHI:.4f}", font_size=22, color=GOLD)
        phi_label.next_to(phi_dot, UP + RIGHT, buff=0.12)

        plastic_dot = Dot(axes.c2p(3, PLASTIC), color=GREEN, radius=0.09)
        plastic_label = Text(f"P ~= {PLASTIC:.4f}", font_size=22, color=GREEN)
        plastic_label.next_to(plastic_dot, DOWN + RIGHT, buff=0.12)

        title = Text("The Golden Curve:  x^n = x + 1", font_size=30)
        title.to_edge(UP)

        self.add(
            title,
            axes, x_label, y_label,
            curve, asymptote,
            phi_dot, phi_label,
            plastic_dot, plastic_label,
        )
        self.wait()
```

- [ ] **Step 2: Render final static image**

```bash
manim -s --output_file golden_curve scene.py GoldenCurveScene
```

Verify: title at top, gold dot at (2, phi), green dot at (3, P), smooth blue curve, dashed asymptote.

**Bonus — animated version:** Replace the `self.add(...) / self.wait()` block with:

```python
        self.play(Write(title))
        self.play(Create(axes), Write(x_label), Write(y_label))
        self.play(Create(curve), Create(asymptote), run_time=3)
        self.play(FadeIn(phi_dot), Write(phi_label))
        self.play(FadeIn(plastic_dot), Write(plastic_label))
        self.wait(2)
```

Then render without `-s`: `manim scene.py GoldenCurveScene` — produces an MP4.

- [ ] **Step 3: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 02-python-manim/scene.py
git commit -m "feat(L2): add labeled key points, title, asymptote"
```

---

# Lesson 3 — Rust + PyO3 + Manim

---

### Task 10: Scaffold the PyO3 project

**Files:**
- Create: `03-rust-pyo3-manim/Cargo.toml`
- Create: `03-rust-pyo3-manim/src/main.rs`
- Create: `03-rust-pyo3-manim/requirements.txt`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "golden_curve_pyo3"
version = "0.1.0"
edition = "2021"

[dependencies]
pyo3 = "0.22"
```

- [ ] **Step 2: Create src/main.rs placeholder**

```rust
fn main() {
    println!("Lesson 3: Rust + PyO3 + Manim");
}
```

- [ ] **Step 3: Create requirements.txt**

```
manim
```

- [ ] **Step 4: Set the Python interpreter for PyO3**

PyO3 needs to know which Python to compile against. Find your Python path:

```bash
where python
```

Then set the environment variable before building (PowerShell):

```powershell
$env:PYO3_PYTHON = (Get-Command python).Source
```

Or bash:
```bash
export PYO3_PYTHON=$(where python | head -1 | tr -d '\r')
```

- [ ] **Step 5: Verify it builds**

```bash
cd C:/Users/charl/The-Golden-Curve/03-rust-pyo3-manim
cargo build
```

Expected: Downloads pyo3, compiles. May take a minute on first build.

- [ ] **Step 6: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 03-rust-pyo3-manim/
git commit -m "feat(L3): scaffold Rust+PyO3 project"
```

---

### Task 11: Rust math functions

**Files:**
- Modify: `03-rust-pyo3-manim/src/main.rs`

- [ ] **Step 1: Write the failing tests**

```rust
fn main() {
    println!("Lesson 3: Rust + PyO3 + Manim");
}

fn newton(n: f64) -> f64 {
    todo!()
}

fn solve_for_n(x: f64) -> f64 {
    todo!()
}

fn generate_curve(x_min: f64, x_max: f64, steps: usize) -> Vec<(f64, f64)> {
    todo!()
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
```

- [ ] **Step 2: Implement all three functions**

```rust
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
```

- [ ] **Step 3: Run tests**

```bash
cargo test
```

Expected: `test result: ok. 4 passed; 0 failed`

- [ ] **Step 4: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 03-rust-pyo3-manim/src/main.rs
git commit -m "feat(L3): implement Rust math functions"
```

---

### Task 12: PyO3 "hello from Python" — first bridge

> **Key concept:** `Python::with_gil(|py| {...})` acquires the GIL (Global Interpreter Lock — Python's global mutex ensuring only one thread runs Python bytecode at a time). Inside the closure, `py` is a token proving you hold it. All PyO3 operations require this token.

**Files:**
- Modify: `03-rust-pyo3-manim/src/main.rs`

- [ ] **Step 1: Add PyO3 import and hello-world bridge to main**

```rust
use pyo3::prelude::*;

fn main() {
    // Initialize Python before acquiring the GIL
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        // Import sys and read the version attribute directly
        let sys = py.import("sys").expect("Could not import sys");
        let version: String = sys
            .getattr("version")
            .expect("No version attr")
            .extract()
            .expect("Could not read version string");
        println!("Python version: {version}");
    });
}
```

- [ ] **Step 2: Build and run**

```bash
cargo run
```

Expected: `Python version: 3.11.x (...)` (your installed Python)

If you get a linker error about Python, double-check `PYO3_PYTHON` is set.

- [ ] **Step 3: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 03-rust-pyo3-manim/src/main.rs
git commit -m "feat(L3): PyO3 hello-world bridge working"
```

---

### Task 13: Create `render.py` — Manim driven programmatically

**Files:**
- Create: `03-rust-pyo3-manim/render.py`

> **Key concept:** Manim's `Scene.render()` can be called from plain Python — no CLI needed. Configure it via `manim.config` before instantiating the scene. This is how we receive data from Rust and hand it to Manim without spawning a subprocess.

- [ ] **Step 1: Create `render.py`**

```python
"""
render.py -- called from Rust via PyO3.
Receives pre-computed curve data from Rust and renders with Manim.
"""


def render_scene(points: list) -> None:
    """
    points: list of (n, x) tuples, already computed by Rust.
    Renders the Golden Curve to a static PNG using Manim.
    """
    from manim import config
    config.save_last_frame = True
    config.output_file = "golden_curve_pyo3"
    config.disable_caching = True
    config.verbosity = "WARNING"

    from manim import (
        Scene, Axes, Dot, Text, DashedLine,
        VMobject, BLUE, GOLD, GREEN, GRAY, UP, RIGHT, DOWN,
    )
    import math

    PHI     = (1 + math.sqrt(5)) / 2
    PLASTIC = 1.3247179572447460

    # Filter to n in [2, 20] and sort by n
    display = sorted([(n, x) for n, x in points if 2.0 <= n <= 20.0])

    class GoldenCurvePyO3Scene(Scene):
        def construct(self):
            axes = Axes(
                x_range=[2, 20, 2],
                y_range=[1, PHI + 0.1, 0.1],
                x_length=10,
                y_length=5.5,
                axis_config={"include_tip": False},
            )
            x_label = axes.get_x_axis_label(Text("n", font_size=28))
            y_label = axes.get_y_axis_label(Text("x", font_size=28))

            # Build smooth curve from Rust-supplied (n, x) pairs
            curve = VMobject(color=BLUE, stroke_width=3)
            scene_pts = [axes.c2p(n, x) for n, x in display]
            curve.set_points_smoothly(scene_pts)

            asymptote = DashedLine(
                start=axes.c2p(2, 1), end=axes.c2p(20, 1),
                dash_length=0.15, color=GRAY, stroke_width=1.5,
            )

            phi_dot = Dot(axes.c2p(2, PHI), color=GOLD, radius=0.09)
            phi_label = Text(f"phi ~= {PHI:.4f}", font_size=22, color=GOLD)
            phi_label.next_to(phi_dot, UP + RIGHT, buff=0.12)

            plastic_dot = Dot(axes.c2p(3, PLASTIC), color=GREEN, radius=0.09)
            plastic_label = Text(f"P ~= {PLASTIC:.4f}", font_size=22, color=GREEN)
            plastic_label.next_to(plastic_dot, DOWN + RIGHT, buff=0.12)

            title = Text(
                "The Golden Curve (Rust + PyO3 + Manim)", font_size=26
            ).to_edge(UP)

            self.add(
                title, axes, x_label, y_label,
                curve, asymptote,
                phi_dot, phi_label,
                plastic_dot, plastic_label,
            )
            self.wait()

    GoldenCurvePyO3Scene().render()
    print("Saved: golden_curve_pyo3.png  (check media/images/)")
```

- [ ] **Step 2: Smoke-test render.py directly from Python**

Run from the `03-rust-pyo3-manim/` directory:

```bash
cd C:/Users/charl/The-Golden-Curve/03-rust-pyo3-manim
python -c "
import math
from render import render_scene
pts = [(math.log(x+1)/math.log(x), x) for x in [1.001 + i*0.006 for i in range(100)]]
render_scene(pts)
"
```

Expected: Manim renders, prints `Saved: golden_curve_pyo3.png`. Open the PNG to verify it looks correct before wiring up Rust.

- [ ] **Step 3: Commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 03-rust-pyo3-manim/render.py
git commit -m "feat(L3): add render.py -- Manim scene driven programmatically"
```

---

### Task 14: Full PyO3 integration — Rust hands data to Manim

**Files:**
- Modify: `03-rust-pyo3-manim/src/main.rs`

> **Key concept:** Rust's `Vec<(f64, f64)>` becomes a Python list of tuples via `.into_py(py)`. PyO3 implements `IntoPy<PyObject>` for standard Rust types including `(f64, f64)`, so the conversion is one line.

- [ ] **Step 1: Replace `fn main()`**

```rust
use pyo3::prelude::*;

fn main() {
    pyo3::prepare_freethreaded_python();

    // Rust computes all the math
    let curve = generate_curve(1.001, 1.617, 800);

    // Filter to the display range n in [2, 20]
    let display: Vec<(f64, f64)> = curve
        .into_iter()
        .filter(|(n, _)| *n >= 2.0 && *n <= 20.0)
        .collect();

    println!("Rust computed {} curve points. Handing to Python...", display.len());

    Python::with_gil(|py| {
        // Add current directory to Python path so render.py is importable
        let sys = py.import("sys").expect("Could not import sys");
        sys.getattr("path")
            .unwrap()
            .call_method1("insert", (0, "."))
            .unwrap();

        // Import render.py
        let render_module = py.import("render").expect("Could not import render.py");

        // Convert Vec<(f64, f64)> -> Python list of tuples
        // (f64, f64) implements IntoPy<PyObject>, so .into_py(py) works directly
        let py_points: Vec<PyObject> = display
            .iter()
            .map(|(n, x)| (*n, *x).into_py(py))
            .collect();

        // Call render_scene(points)
        render_module
            .call_method1("render_scene", (py_points,))
            .expect("render_scene failed");
    });
}
```

- [ ] **Step 2: Build**

```bash
cd C:/Users/charl/The-Golden-Curve/03-rust-pyo3-manim
cargo build
```

- [ ] **Step 3: Run from the lesson directory (render.py must be in CWD)**

```bash
cargo run
```

Expected:
```
Rust computed NNN curve points. Handing to Python...
[Manim progress output...]
Saved: golden_curve_pyo3.png  (check media/images/)
```

Open the PNG — it should look identical to Lesson 2 but with "Rust + PyO3 + Manim" in the title.

- [ ] **Step 4: Run all Rust tests**

```bash
cargo test
```

Expected: `test result: ok. 4 passed; 0 failed`

- [ ] **Step 5: Final commit**

```bash
cd C:/Users/charl/The-Golden-Curve
git add 03-rust-pyo3-manim/src/main.rs
git commit -m "feat(L3): full PyO3 integration -- Rust feeds curve data to Manim"
```

---

## Self-review notes

**Spec coverage check:**
- [x] Lesson 1: Rust + plotters, Newton's method, closed form, module split, CLI
- [x] Lesson 2: Python + Manim, solver.py with tests, scene.py, static PNG + animated bonus
- [x] Lesson 3: Rust + PyO3 + Manim, full pipeline
- [x] Math: Newton's method and closed form present in all lessons
- [x] B-level concept (modules): Task 5
- [x] C-level concept (CLI): Task 6
- [x] PyO3 GIL explanation: Tasks 12 and 14
- [x] "Out of scope" items not included (no complex roots, no interactive GUI)

**Type consistency:**
- `newton(n: f64) -> f64` consistent across Tasks 2, 11
- `solve_for_n(x: f64) -> f64` consistent across Tasks 2, 11
- `generate_curve(x_min, x_max, steps) -> Vec<(f64,f64)>` consistent
- Python `generate_curve(x_min, x_max, steps)` consistent across Task 7
- `render_scene(points: list)` matches both Task 13 definition and Task 14 call site

**No placeholders:** verified — all steps contain complete runnable code.

**Known fragility to watch for:**
- Manim output path includes a version string — use `--output_file` to control the name.
- `PYO3_PYTHON` must be set in the shell before the first `cargo build` in Lesson 3; not needed again.
- `DashedLineSeries` in plotters requires version 0.3.6+; if it errors, remove the asymptote line in Lesson 1 for now.
- Manim on Windows: if rendering fails with a cairo or ffmpeg error, run `pip install manim[all]` for the full dependency bundle.
