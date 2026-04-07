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
    print("Saved: check media/images/ for golden_curve_pyo3.png")
