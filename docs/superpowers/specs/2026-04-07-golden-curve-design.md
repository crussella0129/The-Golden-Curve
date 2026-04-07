# The Golden Curve — Design Spec
*Date: 2026-04-07*

## Overview

A Rosetta Stone repo: the same mathematical problem solved three ways, in three
separate lesson folders. The problem is the equation `x^n = x + 1`, whose
continuous solution curve the author calls "The Golden Curve." At `n=2`, `x = φ`
(the golden ratio). At `n=3`, `x` is the plastic constant. As `n → ∞`, `x → 1`.
The distance from 1 to φ is φ — hence the name.

Each lesson is self-contained: its own dependencies, its own entry point, its own
output. The math is identical across all three. The tools are what changes.

---

## The Math (shared across all lessons)

### Equation
```
x^n = x + 1
```

Rearranged for Newton's method:
```
f(x)  = x^n - x - 1 = 0
f'(x) = n·x^(n-1) - 1
```

### Mode A — Given n, solve for x (Newton's method)
```
x₀ = 1.5
x_{k+1} = x_k - f(x_k) / f'(x_k)
converge when |x_{k+1} - x_k| < 1e-12
```
- Valid for all n ≥ 2 (x > 1 guaranteed)
- Converges in ~5 iterations for double precision
- Verified: n=2 → φ ≈ 1.6180339887, n=3 → 1.3247179572

### Mode B — Given x, solve for n (exact closed form, no iteration)
```
n = ln(x + 1) / ln(x)       [requires x > 1]
```
- Derived directly from x^n = x+1 by taking logarithms
- Exact: x=φ gives n=2.0 (because φ²=φ+1 so ln(φ+1)=2·ln(φ))
- Verified: x=1.3247 → n≈3.0

### Curve generation (the "sweep" strategy)
Rather than running Newton's method for hundreds of n values, sweep x from 1.001
to 1.618 (just below φ) and compute n via Mode B for each point. This gives the
full n≥2 curve with no iteration — sweeping past φ would yield n<2, which is out
of scope. Plot with n on the horizontal axis, x on the vertical. The curve:
- Asymptotes to x=1 as n→∞
- Passes through (2, φ) and (3, 1.3247...)
- Is monotonically decreasing

---

## Repo Structure

```
The-Golden-Curve/
├── README.md
├── 01-rust-plotters/        # Lesson 1: Pure Rust + plotters crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── solver.rs        # (introduced as a refactor step)
│       └── plotter.rs       # (introduced as a refactor step)
│
├── 02-python-manim/         # Lesson 2: Pure Python + Manim
│   ├── requirements.txt
│   ├── solver.py
│   └── scene.py
│
└── 03-rust-pyo3-manim/      # Lesson 3: Rust math + PyO3 + Manim viz
    ├── Cargo.toml
    ├── requirements.txt
    └── src/
        └── main.rs
```

---

## Lesson 1: Pure Rust + `plotters`

### Purpose
Introduce Rust's ecosystem for numerical work and static image output. No Python.
No GUI. One binary, one PNG.

### Crates
```toml
[dependencies]
plotters = "0.3"
```

### Learning progression (this is the instruction order)
1. Implement `fn newton(n: f64) -> f64` in `main.rs` — the core solver
2. Implement `fn solve_for_n(x: f64) -> f64` — the closed form
3. Print a table of (n, x) pairs to stdout to verify the math
4. Generate curve points as `Vec<(f64, f64)>` via the sweep strategy
5. Use `plotters` to draw the curve and save `golden_curve.png`
6. **B-level concept:** Refactor solver and plotter into separate modules (`mod solver`, `mod plotter`)
7. **C-level concept:** Add simple CLI via `std::env::args()` — accept `--x <value>` or `--n <value>`

### Output
`golden_curve.png` — line chart, n on x-axis (2..20), x on y-axis (1..2), with
horizontal dashed line at y=φ and a labeled dot at (2, φ) and (3, 1.3247).

---

## Lesson 2: Pure Python + Manim

### Purpose
Same math, minimal friction. Fastest path to a beautiful result. Introduces Manim
(Manim Community Edition) — the library behind 3Blue1Brown's videos.

### Dependencies
```
manim
```
Install: `pip install manim`
Render static PNG: `manim -s scene.py GoldenCurveScene`

### Files
**`solver.py`** — pure math, no imports beyond `math`:
- `newton(n, x0=1.5, tol=1e-12, max_iter=100) -> float`
- `solve_for_n(x: float) -> float`
- `generate_curve(x_min=1.001, x_max=1.618, steps=500) -> list[tuple[float, float]]`

**`scene.py`** — Manim scene:
```python
from manim import *
from solver import generate_curve, newton

class GoldenCurveScene(Scene):
    def construct(self):
        # 1. Create Axes
        # 2. Plot the curve via ax.plot(lambda x: ...)
        # 3. Add labeled dots at (n=2, x=φ) and (n=3, x=plastic)
        # 4. Add MathTex labels
        # 5. self.wait() — triggers last-frame capture for -s flag
```

### Learning progression
1. Write `solver.py` — implement and test all three math functions
2. Verify by printing: `newton(2)` should give φ, `solve_for_n(1.6180339887)` should give 2.0
3. Write the Manim scene skeleton (Axes + placeholder)
4. Add `ax.plot()` with the curve lambda
5. Add dots and labels for key points
6. Render: `manim -s scene.py GoldenCurveScene`

### Key Manim concepts introduced
- `Scene` and `construct()`
- `Axes` with custom ranges and labels
- `ax.plot(func, x_range=[...])` for a continuous function
- `Dot`, `MathTex`, `always_redraw`
- `-s` flag for static PNG vs. full video render

---

## Lesson 3: Rust + PyO3 + Manim (My Recommendation)

### Purpose
Rust owns the math. Python (via PyO3) owns the visualization. This is the
"right tool for each job" pattern — the same split used in production systems
like HuggingFace Tokenizers (Rust core, Python bindings). Introduces cross-
language FFI without leaving Cargo.

### Crates
```toml
[dependencies]
pyo3 = { version = "0.22", features = ["auto-initialize"] }
```

### Dependencies
Also needs: `pip install manim` (same as Lesson 2)

### Architecture
```
main.rs
  │
  ├── newton(n: f64) -> f64          ← pure Rust
  ├── solve_for_n(x: f64) -> f64     ← pure Rust
  ├── generate_curve() -> Vec<...>   ← pure Rust
  │
  └── Python::with_gil(|py| { ... }) ← PyO3 bridge
        │
        └── calls render.py (lesson 3's own Python helper)
            which receives curve data and drives Manim
```

### Learning progression
1. Implement all three math functions in Rust (same as Lesson 1, step 1-4)
2. Add PyO3 to Cargo.toml and write `Python::with_gil(...)` — just print "hello from Python" first
3. Pass the curve `Vec<(f64, f64)>` into Python as a list of tuples
4. Write a small Python helper (`render.py`) that accepts curve data and calls Manim
5. Call `render.py` from Rust via PyO3

### Key PyO3 concepts introduced
- `Python::with_gil()` — acquiring the GIL (Global Interpreter Lock)
- Converting Rust types to Python objects (`ToPyObject`, `IntoPy`)
- `py.import()` to call Python modules from Rust
- Why the GIL exists and what it means for safety

### Output
Same PNG as Lessons 1 and 2 — the visual result is identical. The point is
the architecture, not the image.

---

## Cross-Lesson Observations (for the README)

| | Lesson 1 | Lesson 2 | Lesson 3 |
|---|---|---|---|
| Language | Rust | Python | Rust + Python |
| Math impl | Rust | Python | Rust |
| Visualization | plotters (Rust) | Manim | Manim via PyO3 |
| Setup complexity | Low | Medium | High |
| Output | PNG | PNG or MP4 | PNG or MP4 |
| Key concept | Ownership, modules | Scene graph, animation | Cross-language FFI |

---

## What is not in scope

- Interactive GUI or web frontend (future extension)
- Complex number solutions (only real positive roots)
- Formal error handling / CLI polish
- Any solution for n < 2 (curve asymptotes naturally, not worth special-casing)
