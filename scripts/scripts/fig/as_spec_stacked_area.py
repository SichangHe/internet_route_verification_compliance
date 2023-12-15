"""Run at `scripts/` with `python3 -m scripts.fig.as_spec_stacked_area`.
"""
import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from numpy import NaN
from scripts.csv_files import as_stats
from scripts.fig import smart_sample

FILE = as_stats
PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")
TAGS = (
    "spec_export_customers",
    "spec_as_is_origin_but_no_route",
    "spec_as_set_contains_origin_but_no_route",
    "spec_import_from_neighbor",
    "spec_uphill",
    "spec_uphill_tier1",
    "spec_tier1_pair",
    "spec_import_peer_oifps",
    "spec_import_customer_oifps",
)


def plot():
    df = pd.read_csv(FILE.path)

    d = pd.DataFrame(
        {
            "total_spec": sum(df[tag] for tag in TAGS),
            "total_report": sum(
                df[f"{port}_{level}"] for port in PORTS for level in LEVELS
            ),
        }
    )
    d["spec_rate"] = d["total_spec"] / d["total_report"]
    d["%non_spec"] = 100.0 - (d["spec_rate"] * 100.0)
    for tag in TAGS:
        d[f"%{tag}"] = df[tag] / d["total_spec"] * 100.0 * d["spec_rate"]
    d.replace(NaN, 0.0, inplace=True)
    d.sort_values(
        by=["%non_spec"] + [f"%{tag}" for tag in TAGS],
        ascending=[True] + [False for _ in TAGS],
        ignore_index=True,
        inplace=True,
    )
    indexes, values = smart_sample(
        tuple(d[f"%{tag}"] for tag in TAGS),
        min_gap_frac=0.0003,
    )

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        labels=[f"%{tag}" for tag in TAGS],
    )
    ax.set_xlabel("AS", fontsize=16)
    ax.set_ylabel(f"Percentage of Special Case", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="lower center", fontsize=14)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    FILE.download_if_missing()
    fig, _, _ = plot()

    pdf_name = f"AS-special-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
