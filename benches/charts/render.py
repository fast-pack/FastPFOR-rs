#!/usr/bin/env python3
"""Renders the README's Rust vs C++ cycle charts from runs saved by `just bench perf ... --save <file>`.

Usage: benches/charts/render.py target/bench/run-1.tsv target/bench/run-2.tsv ...

Each chart shows the median cycles per value over the runs, with a whisker from the fastest to the
slowest run, for Rust (default `Auto` kernels) and C++. It writes a light and a dark SVG per chart
next to this script; the README picks one with `<picture>`.
"""

import statistics
import sys
from pathlib import Path

VALUES = 131172  # values per run, see `N_VALUES` in benches/instructions.rs
ROWS = [
    (layout, width, block)
    for layout in ("Sequential", "Interleaved")
    for width in ("u32", "u64")
    for block in ("128", "256")
]
SERIES = [("Rust", "Auto"), ("C++", "C")]
THEMES = {
    "light": {
        "surface": "#fcfcfb",
        "text": "#0b0b0b",
        "secondary": "#52514e",
        "muted": "#898781",
        "grid": "#e1e0d9",
        "series": ["#2a78d6", "#eb6834"],
    },
    "dark": {
        "surface": "#1a1a19",
        "text": "#ffffff",
        "secondary": "#c3c2b7",
        "muted": "#898781",
        "grid": "#2c2c2a",
        "series": ["#3987e5", "#d95926"],
    },
}


def load(paths):
    """Returns `{(case, op): [cycles per value of each run]}`."""
    runs = {}
    for path in paths:
        for line in Path(path).read_text().splitlines():
            case, op, metric, count = line.split("\t")
            if metric == "1":  # 0: instructions, 1: cycles
                runs.setdefault((case, op), []).append(int(count) / VALUES)
    return runs


def nice_step(top):
    for step in (0.25, 0.5, 1, 2, 2.5, 5):
        if top / step <= 6:
            return step
    return 10


def chart(runs, op, title, theme):
    t = THEMES[theme]
    width, left, right, top = 760, 190, 70, 86
    bar, gap, row_gap = 12, 2, 14
    row_h = 2 * bar + gap + row_gap
    plot_w = width - left - right
    height = top + len(ROWS) * row_h + 40

    stats = {}
    for row in ROWS:
        for name, impl in SERIES:
            values = runs.get((f"{'-'.join(row)}-{impl}", op))
            if values:
                stats[(row, name)] = (statistics.median(values), min(values), max(values))
    peak = max(hi for _, _, hi in stats.values())
    step = nice_step(peak)
    axis_max = step * (int(peak / step) + 1)
    x = lambda v: left + v / axis_max * plot_w

    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}" font-family="-apple-system, Segoe UI, Helvetica, Arial, sans-serif">',
        f'<rect width="{width}" height="{height}" rx="8" fill="{t["surface"]}"/>',
        f'<text x="24" y="32" font-size="17" font-weight="600" fill="{t["text"]}">{title}</text>',
        f'<text x="24" y="54" font-size="12.5" fill="{t["secondary"]}">CPU cycles per value, '
        f"median of {len(next(iter(runs.values())))} runs; whiskers span the fastest to the slowest run. "
        "Lower is better.</text>",
    ]
    # Legend.
    lx = left
    for i, (name, _) in enumerate(SERIES):
        out.append(f'<rect x="{lx}" y="66" width="12" height="12" rx="3" fill="{t["series"][i]}"/>')
        out.append(f'<text x="{lx + 18}" y="76.5" font-size="12.5" fill="{t["text"]}">{name}</text>')
        lx += 70
    # Gridlines and ticks.
    plot_bottom = top + len(ROWS) * row_h - row_gap / 2
    v = 0.0
    while v <= axis_max + 1e-9:
        gx = x(v)
        out.append(f'<line x1="{gx:.1f}" y1="{top - 6}" x2="{gx:.1f}" y2="{plot_bottom:.1f}" '
                   f'stroke="{t["grid"]}" stroke-width="1"/>')
        out.append(f'<text x="{gx:.1f}" y="{plot_bottom + 18:.1f}" font-size="11.5" text-anchor="middle" '
                   f'fill="{t["muted"]}">{v:g}</text>')
        v += step
    # Rows.
    for r, row in enumerate(ROWS):
        y0 = top + r * row_h
        layout, width_, block = row
        if r % 4 == 0:
            out.append(f'<text x="24" y="{y0 + bar + 5}" font-size="12.5" font-weight="600" '
                       f'fill="{t["text"]}">{layout}</text>')
        out.append(f'<text x="{left - 12}" y="{y0 + bar + 5}" font-size="12.5" text-anchor="end" '
                   f'fill="{t["secondary"]}">{width_} × {block}</text>')
        for i, (name, _) in enumerate(SERIES):
            by = y0 + i * (bar + gap)
            if (row, name) not in stats:
                out.append(f'<text x="{left + 6}" y="{by + bar - 2}" font-size="11.5" '
                           f'fill="{t["muted"]}">{name} has no 64-bit SIMDFastPFor</text>')
                continue
            med, lo, hi = stats[(row, name)]
            w = x(med) - left
            # Data-end rounded, anchored flat on the baseline.
            out.append(
                f'<path d="M{left},{by} h{w - 4:.1f} a4,4 0 0 1 4,4 v{bar - 8} a4,4 0 0 1 -4,4 '
                f'h{-(w - 4):.1f} z" fill="{t["series"][i]}"/>'
            )
            cy = by + bar / 2
            out.append(f'<line x1="{x(lo):.1f}" y1="{cy}" x2="{x(hi):.1f}" y2="{cy}" '
                       f'stroke="{t["text"]}" stroke-opacity="0.55" stroke-width="1.5"/>')
            for end in (lo, hi):
                out.append(f'<line x1="{x(end):.1f}" y1="{cy - 3.5}" x2="{x(end):.1f}" y2="{cy + 3.5}" '
                           f'stroke="{t["text"]}" stroke-opacity="0.55" stroke-width="1.5"/>')
            out.append(f'<text x="{x(max(hi, med)) + 6:.1f}" y="{by + bar - 2}" font-size="11.5" '
                       f'fill="{t["secondary"]}">{med:.2f}</text>')
    out.append("</svg>")
    return "\n".join(out) + "\n"


def main():
    runs = load(sys.argv[1:])
    here = Path(__file__).parent
    for op, title in (("encode", "Encoding: Rust vs C++"), ("decode", "Decoding: Rust vs C++")):
        for theme in THEMES:
            (here / f"{op}-{theme}.svg").write_text(chart(runs, op, title, theme))


if __name__ == "__main__":
    main()
