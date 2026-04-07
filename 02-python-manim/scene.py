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

        # axes.plot(f) where f(n) -> x uses the "given n, find x" mode
        curve = axes.plot(
            lambda n: newton(n),
            x_range=[2.01, 19.99, 0.05],
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
