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
