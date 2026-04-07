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
