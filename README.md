# The Golden Curve

> **x^n = x + 1**
>
> At n=2, x is the golden ratio. At n=3, x is the plastic constant. As n → ∞, x → 1. The two most fundamental results — φ and 1 — are themselves in golden ratio.

This repo solves and visualizes a single mathematical equation three ways: pure Rust, pure Python with Manim, and Rust doing the math while Python (via PyO3) handles the visualization. Same problem, three toolchains, side by side.

It's a Rosetta Stone between Rust and Python, a math exploration, and a structured set of exercises. The finished code is here. The instructions below are for rebuilding it yourself — with enough explanation that you understand *why* each piece works, not just *what* it does.

---

## Contents

- [The Math](#the-math)
- [How to use this repo](#how-to-use-this-repo)
- [Lesson 1 — Pure Rust + plotters](#lesson-1--pure-rust--plotters)
- [Lesson 2 — Pure Python + Manim](#lesson-2--pure-python--manim)
- [Lesson 3 — Rust + PyO3 + Manim](#lesson-3--rust--pyo3--manim)
- [Learning notes](#learning-notes)
- [Reference: verified values](#reference-verified-values)

---

## The Math

### The equation

```
x^n = x + 1
```

This is a **family** of equations, one for each value of n. For each n > 1, there is exactly one real solution x > 1. The equation says: *find the number whose nth power exceeds it by exactly 1.*

### Why n = 2 gives the golden ratio

Set n = 2:

```
x^2 = x + 1
x^2 - x - 1 = 0
```

Apply the quadratic formula:

```
x = (1 ± √5) / 2
```

Taking the positive root:

```
φ = (1 + √5) / 2 ≈ 1.6180339887...
```

This is the **golden ratio**. Its defining property is φ² = φ + 1, which is exactly the equation we started with. That property is not a coincidence — it is the equation.

The golden ratio appears throughout geometry, art, and nature. Its continued fraction representation is [1; 1, 1, 1, ...] — the "most irrational" number in a precise sense. The Fibonacci sequence converges to φ as ratios of consecutive terms.

> **Why φ specifically?** Any number where the square exceeds it by 1 must satisfy x² - x - 1 = 0. The quadratic formula always gives two roots for a degree-2 polynomial. The negative root, (1 - √5)/2 ≈ -0.618, is the only other solution, but it's negative — not a valid "size" in the geometric sense. So φ is the unique positive answer.

### Why n = 3 gives the plastic constant

Set n = 3:

```
x^3 = x + 1
x^3 - x - 1 = 0
```

This cubic has no "nice" closed form like the quadratic did (Cardano's formula gives one, but it's ugly). The unique positive real root is:

```
P ≈ 1.3247179572...
```

This is the **plastic constant** (also called the plastic number), named by Dutch architect Dom Hans van der Laan. It satisfies P³ = P + 1, and like φ it appears in geometric proportions and spiral tilings. Where the golden ratio is associated with pentagons and spirals in 2D, the plastic constant has analogous roles in 3D.

Unlike φ, P is not constructible with compass and straightedge — it requires solving a cubic.

### As n → ∞, x → 1

As the exponent grows arbitrarily large, the solution shrinks toward 1. The rate of approach is approximately:

```
x_n ≈ 1 + ln(2) / n
```

Derivation sketch: if x = 1 + ε for small ε, then x^n ≈ e^(nε). Setting e^(nε) = x + 1 ≈ 2 gives nε ≈ ln(2), so ε ≈ ln(2)/n → 0.

> **Why is this called the Golden Curve?** The curve traced by (n, x) as n ranges over [2, ∞) goes from the golden ratio φ at n = 2 down toward 1 as n → ∞. The ratio φ/1 = φ — the two bounding values of the curve are themselves in golden ratio. The curve lives between 1 and its most beautiful point.

### Is x^n = x + 1 "transcendental"?

It depends on how you ask the question.

**If you fix n to an integer** — then x^n - x - 1 = 0 is a *polynomial* equation of degree n. Polynomials are algebraic, not transcendental. Newton's method solves any polynomial exactly (to machine precision). Excel's GoalSeek was correct to use an iterative solver, but the equation itself is not transcendental.

**If you ask about the curve** — the relationship between n and x across the full curve satisfies n = ln(x+1) / ln(x), which involves logarithms. Logarithms are transcendental functions. The *curve*, treated as a continuous object over real-valued n, is transcendental.

So: individual equations in the family are algebraic. The family as a whole is transcendental. Both claims have merit depending on what you're asking.

### Newton's method

Newton's method is the standard algorithm for finding roots of smooth functions. Given f(x) = 0, start with a guess x₀ and iterate:

```
x_{k+1} = x_k - f(x_k) / f'(x_k)
```

**Geometric intuition:** at each step, draw the tangent line to f at the current point. The next guess is where that tangent line crosses zero. Because f is nearly linear near a root (by definition of the derivative), this converges very fast.

**Convergence:** Newton's method is *quadratically convergent* — each iteration roughly doubles the number of correct digits. Starting from x₀ = 1.5 with n = 2, you reach 15 correct digits in 4 iterations.

For x^n = x + 1, define:

```
f(x)  = x^n - x - 1
f'(x) = n · x^(n-1) - 1
```

Starting point: x₀ = 1.5 works for all n ≥ 2 because the root is always in (1, 2).

Stopping condition: stop when `|x_new - x| < 1e-12`. This is well within double precision (which has about 15-16 significant decimal digits).

### The exact closed form for n (no iteration needed)

Given x, you can find n *exactly* — no Newton's method, no iteration:

```
x^n = x + 1
n · ln(x) = ln(x + 1)
n = ln(x + 1) / ln(x)
```

This is a closed-form expression. Plug in x = φ:

```
n = ln(φ + 1) / ln(φ)
  = ln(φ²) / ln(φ)       [because φ + 1 = φ² by definition]
  = 2 · ln(φ) / ln(φ)
  = 2
```

Exact. This is how we generate the full curve without iteration — sweep x, compute n directly.

### The curve in practice

For the visualizations in this repo:

- **Horizontal axis:** n (the exponent), ranging from 2 to 20
- **Vertical axis:** x (the solution), ranging from 1 to φ+margin
- The curve passes through (2, φ) and (3, P)
- A dashed asymptote marks x = 1 (never reached)
- The curve is monotonically decreasing — larger exponents require smaller bases

---

## How to use this repo

### Prerequisites

| Lesson | Required |
|--------|----------|
| 1 (Rust) | Rust + Cargo — install at [rustup.rs](https://rustup.rs) |
| 2 (Python) | Python 3.11+, then `pip install manim pytest` |
| 3 (PyO3) | Both of the above. On Windows, set `PYO3_PYTHON` before building (see lesson) |

### Structure

```
The-Golden-Curve/
├── 01-rust-plotters/      # Lesson 1: Pure Rust
├── 02-python-manim/       # Lesson 2: Pure Python + Manim
└── 03-rust-pyo3-manim/    # Lesson 3: Rust math, Manim viz, PyO3 bridge
```

Each lesson is self-contained. Run tests from within the lesson directory.

### Running the finished code

**Lesson 1:**
```bash
cd 01-rust-plotters
cargo test          # 5 tests
cargo run           # prints table, saves golden_curve.png
cargo run -- --n 2  # prints x = 1.6180339887498949
cargo run -- --x 1.618  # prints n = 2.0000...
```

**Lesson 2:**
```bash
cd 02-python-manim
pytest tests/ -v    # 6 tests
manim -s --output_file golden_curve scene.py GoldenCurveScene
```

**Lesson 3:**
```bash
cd 03-rust-pyo3-manim
# On Windows PowerShell first:
# $env:PYO3_PYTHON = (Get-Command python).Source
cargo test          # 4 tests
cargo run           # Rust computes, Python renders
```

---

## Lesson 1 — Pure Rust + plotters

**What you'll learn:** Rust's type system for numerical work, the `fn` / `let` / `for` / `if` basics, cargo project structure, plotting with `plotters`, and how to split code into modules.

**Finished code:** [`01-rust-plotters/src/`](01-rust-plotters/src/)

---

### Concept: Why Rust for math?

Rust has no garbage collector. Memory is managed through *ownership* — a system of rules the compiler enforces at compile time. This means:

- No null pointer crashes at runtime
- No use-after-free bugs
- Zero-cost abstractions: high-level code compiles to the same machine instructions as hand-written C

For numerical work, this matters because floating-point operations are cheap, but incorrect memory management can corrupt results silently. Rust makes corruption impossible by construction.

> **Key mental model:** In Rust, every value has exactly one *owner*. When the owner goes out of scope, the value is dropped (freed). Functions can *borrow* values via references (`&T`), but can't own them unless you move or clone.

**Where to search if you get confused:**
- Stack Overflow: `"rust borrow checker" site:stackoverflow.com`
- Stack Overflow: `"rust ownership explained"`
- Official: [doc.rust-lang.org/book/ch04-00-understanding-ownership.html](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) — Chapter 4 of *The Rust Book* is the canonical ownership explanation

---

### Step-by-step: recreate Lesson 1

#### Step 0: Before you start — predict

Before writing a line of code, answer these in your head (look up what you don't know):

1. Newton's method formula from memory: x_new = ?
2. What is f(x) for the equation x^n = x + 1?
3. What is f'(x)?
4. What value should `newton(2.0)` return?

If you can't answer these without looking up the code, re-read [The Math](#the-math) section first. The code is just an expression of the math — understanding the math first makes the code obvious.

---

#### Step 1: Create the project

```bash
cargo new --name golden_curve 01-rust-plotters
cd 01-rust-plotters
```

**Why `--name`?** Without it, the binary name would be `01-rust-plotters` (with hyphens), which Rust accepts but is awkward. `--name` lets you name the binary independently of the directory.

Add to `Cargo.toml`:

```toml
[workspace]
# ^ This declares the folder a standalone workspace root.
# Without it, Cargo walks up the directory tree looking for a parent
# workspace, which can cause confusing errors if one exists above.

[dependencies]
plotters = "0.3"
```

**Build now, before writing any code:**

```bash
cargo build
```

If this fails, the error is about setup, not your math. Fix setup errors before logic errors — they're different problems.

---

#### Step 2: Write the tests before the functions (TDD)

Test-Driven Development means writing a test that *fails*, then writing the minimum code to make it pass. This is not just ceremony — it forces you to define what "correct" means before you implement anything.

In `src/main.rs`, add:

```rust
fn newton(n: f64) -> f64 {
    todo!()  // todo!() panics at runtime but compiles — lets tests run and fail
}

fn solve_for_n(x: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHI: f64 = 1.6180339887498948;
    const PLASTIC: f64 = 1.3247179572447460;

    #[test]
    fn test_newton_n2_gives_phi() {
        let result = newton(2.0);
        assert!((result - PHI).abs() < 1e-10, "got {result}");
    }
    // ... (add all 4 tests from the source)
}
```

Run: `cargo test` — expect FAILED with "not yet implemented". That's the goal. Red before green.

**What `#[cfg(test)]` means:** This attribute tells the compiler to only include the `mod tests` block when running `cargo test`. The tests don't end up in your release binary.

**What `use super::*` means:** The test module is a child of the main module. `super` refers to the parent, `*` imports everything public from it. Since `newton` and `solve_for_n` are in the parent, this makes them available in the test.

**Where to search:**
- `"rust cfg test attribute" site:stackoverflow.com`
- `"rust #[test] macro explained"`
- For assertion failures: `"rust assert_eq vs assert site:stackoverflow.com"`

---

#### Step 3: Implement Newton's method

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
    x
}
```

**Walk through the unfamiliar syntax:**

- `let mut x` — variables in Rust are immutable by default. `mut` opts into mutability. This is the opposite of most languages. If you forget `mut`, the compiler will tell you exactly where and why.
- `1.5_f64` — the `_f64` suffix tells the compiler this literal is a 64-bit float. Without it, Rust might infer the wrong type.
- `x.powf(n)` — `f64` has methods. `powf` is "power, floating-point exponent". For integer exponents there's `powi`.
- `0..100` — a range. `for _ in 0..100` runs 100 times, discarding the index (`_` means "I don't need this value").
- `return x_new;` — early return from inside a loop. In Rust, the last expression in a function is implicitly returned (no semicolon), but `return` works fine inside control flow.

> **Before running tests:** trace through the first Newton iteration by hand for n=2, x₀=1.5. What is f(1.5)? What is f'(1.5)? What is x₁? Compare to φ = 1.618. This builds intuition that the code implements the math correctly, not just that the tests pass.

Run: `cargo test` — expect the two Newton tests to pass now.

**Where to search:**
- `"rust f64 methods powf powi difference"`
- `"rust mut immutable by default why"`
- `"rust return vs no semicolon"`

---

#### Step 4: Implement the closed form

```rust
fn solve_for_n(x: f64) -> f64 {
    (x + 1.0).ln() / x.ln()
}
```

Three things in one line:
- `(x + 1.0)` — parentheses force evaluation order
- `.ln()` — natural logarithm, a method on `f64`
- Division — standard `/` operator

**Verify mentally before running:** plug in x = 1.6180339887. Does (x+1).ln() / x.ln() = 2? Use a calculator if you need to. The point is to confirm your understanding of the math before trusting the test.

Run: `cargo test` — all 4 tests should pass.

---

#### Step 5: Curve generation + verification table

See [`solver.rs`](01-rust-plotters/src/solver.rs) for the `generate_curve` function.

The key insight: sweep x from 1.001 to 1.617 (just below φ), compute n = ln(x+1)/ln(x) for each. The curve emerges from the closed form — no Newton's method needed for the plot.

After implementing, run `cargo run` and check the table:

| n | x (expected) |
|---|---|
| 2 | 1.6180339887498949 |
| 3 | 1.3247179572447461 |
| 10 | 1.0757660660868371 |
| 20 | 1.0361937171306834 |

If your values match to 10+ digits, the math is right.

---

#### Step 6: Visualization with plotters

Plotters uses a **builder pattern** — a chain of method calls that configure and construct the chart before drawing anything. The pattern:

```
BitMapBackend → into_drawing_area → ChartBuilder → build_cartesian_2d → configure_mesh → draw_series
```

**Why a builder pattern?** A chart has many optional settings (title, margins, axis labels, fonts, colors). If these were all constructor arguments, you'd have a function with 20 parameters. Builders let each setting be optional with a readable name.

See [`plotter.rs`](01-rust-plotters/src/plotter.rs) for the full implementation.

**Key things to understand in plotters:**
- `build_cartesian_2d(x_range..y_range)` — sets the data coordinate system. Your data coordinates (n and x values) are separate from pixel coordinates.
- `LineSeries::new(points, style)` — draws a connected line through a list of (x, y) data points
- `PointSeries::of_element(...)` — draws a custom shape at each data point
- `EmptyElement::at(c) + Circle::new(...)` — composes drawing primitives using operator overloading (Rust lets you define what `+` means for your types)

**Where to search:**
- `"plotters rust ChartBuilder example" site:github.com`
- `"plotters rust LineSeries draw" site:docs.rs`
- docs.rs/plotters — the API reference has examples inline

---

#### Step 7 (B-level): Module refactor

When all code lives in `main.rs`, it becomes hard to test individual pieces in isolation. Rust's module system lets you split by responsibility.

```
src/
├── main.rs       # entry point only — calls other modules
├── solver.rs     # math: newton, solve_for_n, generate_curve
└── plotter.rs    # visualization: plot_curve
```

In `main.rs`, declare the modules:

```rust
mod solver;
mod plotter;
```

The compiler automatically looks for `src/solver.rs` when it sees `mod solver;`. Functions in `solver.rs` that you want visible from `main.rs` must be marked `pub`.

**Mental model:** modules in Rust are like namespaces with explicit visibility. Everything is private by default. `pub` is a deliberate declaration that something is part of the module's public interface.

**Exercises — try these before looking at the source:**
1. What happens if you forget `pub` on `newton`? What error does the compiler give you? (Try it deliberately.)
2. Can `plotter.rs` call functions from `solver.rs` directly? What do you need to write to make that work?

---

#### Step 8 (C-level): CLI with `std::env::args()`

```rust
let args: Vec<String> = std::env::args().collect();
```

`std::env::args()` returns an *iterator* of command-line arguments. `.collect()` gathers an iterator into a collection — here, a `Vec<String>`. The first element (`args[0]`) is always the binary path.

The rest is just searching the collected vector for `--n` or `--x` flags. No external crates needed.

**Where to search:**
- `"rust std::env::args() example"`
- `"rust parse f64 from string" site:stackoverflow.com`
- For a more complete CLI: search `"clap rust arg parsing"` — the `clap` crate is the standard for production CLIs

---

## Lesson 2 — Pure Python + Manim

**What you'll learn:** Python's math library, pytest, and Manim's scene graph model for mathematical animation.

**Finished code:** [`02-python-manim/`](02-python-manim/)

---

### Concept: What is Manim?

Manim (Manim Community Edition) is the library behind 3Blue1Brown's math videos. It models a visualization as a **scene** — a Python class with a `construct()` method that describes what objects exist and how they animate.

The key abstraction: everything is a **Mobject** (Mathematical Object). Axes, curves, dots, text, arrows — all Mobjects. A `Scene` is a container that manages and renders Mobjects.

Manim has two output modes:
- `manim -s scene.py SceneName` — saves the **last frame** as a PNG (static image)
- `manim scene.py SceneName` — renders a full video (MP4)

**Where to search:**
- [docs.manim.community](https://docs.manim.community) — official docs, start here
- `"manim axes plot example" site:docs.manim.community`
- `"manim scene construct method explained"`
- For errors: search the exact error message in quotes on Stack Overflow

---

### Step-by-step: recreate Lesson 2

#### Step 1: Setup

```bash
mkdir 02-python-manim && cd 02-python-manim
pip install manim pytest
manim --version  # should print Manim Community v0.x.x
```

If Manim installs but `manim --version` fails, your Python Scripts directory may not be on PATH. Search: `"pip install command not found windows" site:stackoverflow.com`.

#### Step 2: Write and test `solver.py` first

**Crucially:** implement the Python math functions *before* touching Manim. This separates two problems:
1. Is the math correct?
2. Does the visualization work?

Debugging both at once is much harder than debugging each independently.

The Python solver is nearly identical to the Rust one, just different syntax:

```python
def newton(n: float, x0: float = 1.5, tol: float = 1e-12, max_iter: int = 100) -> float:
    """Given n, find x such that x^n = x + 1."""
    x = x0
    for _ in range(max_iter):
        fx  = x**n - x - 1
        dfx = n * x**(n - 1) - 1
        x_new = x - fx / dfx
        if abs(x_new - x) < tol:
            return x_new
        x = x_new
    return x
```

**Compare to the Rust version line by line.** Notice:
- Python uses `x**n` where Rust uses `x.powf(n)` — same operation, different syntax
- Python has no type annotations required (`x: float` is optional documentation, not enforced)
- Python uses `abs()` as a global function; Rust uses `.abs()` as a method on `f64`
- Python iterates with `range(max_iter)`; Rust with `0..max_iter`

This side-by-side comparison is the point of the Rosetta Stone format. Same algorithm, different idioms.

Run tests before moving on:

```bash
pytest tests/ -v
```

All 6 tests must pass before writing `scene.py`. If any fail, the visualization will show wrong values.

**Where to search:**
- `"python type hints float explained"`
- `"pytest assert example" site:stackoverflow.com`
- `"python math.log natural logarithm"`

---

#### Step 3: Build the Manim scene

The structure of every Manim scene:

```python
from manim import *

class YourScene(Scene):
    def construct(self):
        # Everything you add or animate goes here
        self.add(...)   # adds without animation (use with -s flag)
        self.play(...)  # adds with animation (use without -s flag)
        self.wait()     # holds last frame
```

**The coordinate system:** `Axes` creates a 2D coordinate system in the scene. But Manim has *two* coordinate spaces:
- **Data space:** your mathematical values (n = 2..20, x = 1..1.7)
- **Scene space:** the actual screen coordinates (roughly -7 to 7 horizontally, -4 to 4 vertically)

`axes.c2p(n, x)` converts data space → scene space ("coordinate to point"). You need this whenever you want to place a Dot or label at a specific data coordinate.

`axes.plot(func, x_range=[...])` samples a function at regular intervals and draws a smooth curve. The function takes a data-space x value (n in our case) and returns a data-space y value (x in our case):

```python
curve = axes.plot(
    lambda n: newton(n),      # newton(n) returns the x-value for that n
    x_range=[2.01, 19.99, 0.05],  # [start, stop, step]
    color=BLUE,
)
```

> **Stop and predict:** if you wrote `lambda n: solve_for_n(n)` instead of `lambda n: newton(n)`, what would the output look like? Would it error? Would it draw something wrong? Think through what `solve_for_n` expects as input before answering.

**Where to search:**
- `"manim axes c2p coordinate to point example"`
- `"manim plot lambda function"`
- `"manim Dot next_to buff"`

---

#### Render and inspect

```bash
manim -s --output_file golden_curve scene.py GoldenCurveScene
```

The PNG appears in `media/images/`. Open it and verify:
- Curve descends from (2, φ) toward the bottom right
- Gold dot at n=2, labeled φ
- Green dot at n=3, labeled P
- Dashed line near x=1

**If the curve looks wrong:** the most common mistakes are:
1. Axes range is off — double-check `x_range` and `y_range`
2. `lambda` arguments are swapped — `axes.plot(lambda x: f(x))` where `x` is the horizontal axis variable
3. Data and scene coordinates mixed up — use `axes.c2p()` for explicit placement, never raw scene coordinates

---

#### Bonus: animation

Replace `self.add(...)` with `self.play(...)` and run without `-s` for a full animation:

```python
self.play(Create(axes), Write(x_label), Write(y_label))
self.play(Create(curve), Create(asymptote), run_time=3)
self.play(FadeIn(phi_dot), Write(phi_label))
```

**Where to search:**
- `"manim Create vs Write difference"`
- `"manim run_time animate speed"`
- `"manim FadeIn FadeOut" site:docs.manim.community`

---

## Lesson 3 — Rust + PyO3 + Manim

**What you'll learn:** How to call Python from Rust using PyO3, what the GIL (Global Interpreter Lock) is and why it exists, and how to pass data across a language boundary.

**Finished code:** [`03-rust-pyo3-manim/`](03-rust-pyo3-manim/)

**The architecture:**

```
Rust (main.rs)
  │  computes: newton(), solve_for_n(), generate_curve()
  │  produces: Vec<(f64, f64)> — 754 curve points
  │
  └─ PyO3 bridge (Python::with_gil)
        │  converts: Vec<(f64, f64)> → Python list of tuples
        │
        └─ Python (render.py)
              │  imports: manim
              │  drives: GoldenCurvePyO3Scene().render()
              └─ saves: golden_curve_pyo3.png
```

This is how real systems are built. HuggingFace's tokenizers library is Rust with Python bindings via PyO3. NumPy's core is C with Python bindings. The pattern: use the fast, safe language for computation; use Python for its rich ecosystem of tools.

---

### Concept: The GIL

Python has a **Global Interpreter Lock** — a mutex that only one thread can hold at a time. When Python is running bytecode, it holds the GIL. Nothing else can run Python bytecode simultaneously.

This exists because Python's memory management (reference counting) is not thread-safe. The GIL is the blunt instrument that makes it safe by serializing execution.

When Rust calls into Python via PyO3, it must explicitly acquire this lock:

```rust
Python::with_gil(|py| {
    // py is a token proving you hold the GIL
    // all Python operations require py
});
```

The `py` token is not a pointer or an object — it's a zero-sized type used by the compiler to enforce that you can only call Python APIs when you've acquired the lock. If you try to call a Python API outside `with_gil`, the code won't compile.

> **Why does this matter for learning?** The GIL is the reason Python's threading is limited for CPU-bound work — two threads can't run Python computations simultaneously. This is why PyO3 (Rust doing the CPU work, Python only for the ecosystem) is a genuine architectural improvement, not just a style choice.

**Where to search:**
- `"Python GIL explained simply" site:stackoverflow.com`
- `"why does Python have the GIL" site:realpython.com`
- `"pyo3 Python::with_gil lifetime"`
- `"pyo3 user guide" site:pyo3.rs` — the official PyO3 book

---

### Step-by-step: recreate Lesson 3

#### Step 1: Setup and a critical note about PyO3 0.22

PyO3 0.22 introduced the **bound API**. Several methods were renamed:

| Old (≤0.20) | New (0.21+) |
|---|---|
| `py.import("sys")` | `py.import_bound("sys")` |
| `PyList::new(py, ...)` | `PyList::new_bound(py, ...)` |
| `PyTuple::new(py, ...)` | `PyTuple::new_bound(py, ...)` |

If you search Stack Overflow for PyO3 examples and find code using the old API, it will compile but with deprecation warnings. Prefer the `_bound` variants.

**Where to search:**
- `"pyo3 0.22 bound api migration"`
- `"pyo3 import_bound" site:github.com`

**Tell PyO3 which Python to use:**

```powershell
# Windows PowerShell (run before cargo build)
$env:PYO3_PYTHON = (Get-Command python).Source
```

```bash
# bash/zsh
export PYO3_PYTHON=$(which python)
```

This only needs to be set once per terminal session. PyO3's build script uses it to find the right Python headers to link against.

**Where to search:**
- `"PYO3_PYTHON environment variable" site:pyo3.rs`
- `"pyo3 link python version mismatch"`

---

#### Step 2: Hello from Python (first bridge)

The simplest possible PyO3 usage:

```rust
use pyo3::prelude::*;

fn main() {
    pyo3::prepare_freethreaded_python();  // must call before with_gil

    Python::with_gil(|py| {
        let sys = py.import_bound("sys").unwrap();
        let version: String = sys.getattr("version").unwrap().extract().unwrap();
        println!("Python version: {version}");
    });
}
```

**Walk through each call:**
- `prepare_freethreaded_python()` — initializes the Python interpreter. Required when calling Python from a Rust main function (as opposed to being called *by* Python).
- `py.import_bound("sys")` — equivalent to `import sys` in Python. Returns a `Bound<'_, PyModule>`.
- `.getattr("version")` — equivalent to `sys.version`. Returns a `Bound<'_, PyAny>` — a generic Python object.
- `.extract::<String>()` — converts the Python object to a Rust `String`. This is where PyO3 crosses the type boundary. If the Python object isn't a string, this returns an `Err`.

**Before running:** predict what Python version this will print. Then run `cargo run` and verify.

---

#### Step 3: Pass data across the boundary

```rust
use pyo3::types::{PyList, PyTuple};

let py_points = PyList::new_bound(
    py,
    display.iter().map(|(n, x)| PyTuple::new_bound(py, [*n, *x])),
);
```

**Why `PyTuple::new_bound` instead of `(n, x).into_py(py)`?**

`into_py` is generic — the compiler needs to know *what Python type* to convert to. For `(f64, f64)`, there are multiple valid conversions (a Python tuple, a Python list, etc.), so the compiler gets confused. `PyTuple::new_bound` is explicit: this is definitely a Python tuple. No ambiguity.

The general pattern for type ambiguity in Rust: if the compiler says "type annotations needed" or "multiple impls found", you're usually calling a generic function where the return type is underspecified. The fix is either to annotate (`let x: TargetType = ...`) or to call a more specific function that returns the exact type you want.

**Where to search:**
- `"pyo3 PyList new_bound example"`
- `"rust type annotations needed error site:stackoverflow.com"`
- `"pyo3 convert rust vec to python list"`

---

#### Step 4: render.py — Manim without the CLI

Normally you run Manim as `manim scene.py SceneName`. But you can also call it programmatically:

```python
from manim import config
config.save_last_frame = True
config.output_file = "my_output"

MyScene().render()  # same as running manim -s
```

This is what `render.py` does. It receives the pre-computed curve data from Rust, builds a Manim scene using that data, and calls `.render()` directly.

The key difference from Lesson 2: the curve is built from Rust-supplied `(n, x)` pairs using `VMobject.set_points_smoothly()`, not from `axes.plot()`. This is because `axes.plot()` needs a callable function — but the data is already computed by Rust. `VMobject` accepts a list of scene-space points directly.

```python
curve = VMobject(color=BLUE, stroke_width=3)
scene_pts = [axes.c2p(n, x) for n, x in display]
curve.set_points_smoothly(scene_pts)
```

**Where to search:**
- `"manim VMobject set_points_smoothly example"`
- `"manim programmatic rendering without CLI"`
- `"pyo3 call python function from rust module"`

---

#### Step 5: Compare all three outputs side by side

Open the three PNG files simultaneously:

| File | Location |
|---|---|
| Lesson 1 | `01-rust-plotters/golden_curve.png` |
| Lesson 2 | `02-python-manim/media/images/scene/golden_curve.png` |
| Lesson 3 | `03-rust-pyo3-manim/media/images/golden_curve_pyo3.png` |

The curve should be identical in all three. The titles, colors, and visual quality differ — plotters produces a more utilitarian chart, Manim produces a cleaner mathematical aesthetic — but the mathematical content is the same.

**Exercise:** Modify one lesson to add a third labeled point at n=4 (x ≈ 1.2207). Then add it to all three. Notice which one is easiest and which is hardest. That asymmetry tells you something about each tool's design priorities.

---

## Learning notes

### How to use this repo effectively

Research consistently shows the following practices produce better long-term retention than passive reading:

**1. Predict before running** (retrieval practice)
Before running any `cargo test` or `pytest` command, write down the expected output. Even a rough prediction ("should see 4 tests pass") forces your brain to commit to an expectation, which makes surprises (and their explanations) more memorable.

**2. Explain it to yourself** (elaborative interrogation)
For each non-obvious line of code, ask "why does this work?" and answer in one sentence — out loud or in a comment. If you can't explain it, you don't understand it yet. Look it up before moving on.

**3. Do the lessons out of order on a second pass** (interleaving)
After completing all three lessons once in order, try implementing the solver again starting from a blank file, alternating between languages. Write the Rust `newton()`, then the Python `newton()`, then compare. This cross-language comparison accelerates understanding of both languages.

**4. Use the tests as a specification** (not just verification)
The test file defines what "correct" means. Read the tests *before* reading the implementation. Try to write an implementation that satisfies the tests without looking at the source. Only then compare your version to the provided one.

**5. Retype, don't copy-paste**
When recreating the exercises, type the code by hand even when looking at the source. This is slow and feels inefficient. It is not — typing forces line-by-line attention in a way that copy-paste does not.

### What to do when you're stuck

**For Rust compilation errors:**
Rust's compiler errors are unusually helpful. Read the full error message — it almost always contains a hint or even a fix. Then search the error code (e.g., `E0283`) at [doc.rust-lang.org/error_codes](https://doc.rust-lang.org/error_codes/) for the full explanation.

**For Stack Overflow searches:**
Good searches are specific. Instead of "rust error", search "rust borrow checker cannot move out of borrowed content f64". Instead of "manim help", search "manim axes plot function not showing". Include the language, the specific thing you're trying to do, and if there's an error, the key phrase from the error message.

**For PyO3 specifically:**
PyO3's API changes between versions. Always check which version you have (`Cargo.toml`) and include the version in your search: "pyo3 0.22 call python function from rust". The [PyO3 user guide](https://pyo3.rs/latest/) is authoritative and has version-specific migration guides.

---

## Reference: verified values

| n | x | Notes |
|---|---|---|
| 2 | 1.6180339887498948... | φ, golden ratio. Exact: (1+√5)/2 |
| 3 | 1.3247179572447460... | Plastic constant P |
| 4 | 1.2207440846057595... | |
| 5 | 1.1673039782614187... | |
| 10 | 1.0757660660868371... | |
| 20 | 1.0361937171306834... | |
| ∞ | → 1 | Asymptote. Rate: ≈ 1 + ln(2)/n |

All values computed by Newton's method with tolerance 1e-12. The Lesson 1 CLI can compute any value:

```bash
cd 01-rust-plotters && cargo run -- --n 7
```

---

*Originally derived by the repository author. Mathematical name "The Golden Curve" coined for the curve's bounding values 1 and φ, which are themselves in golden ratio.*
