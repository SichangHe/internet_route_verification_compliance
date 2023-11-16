"""Run at `scripts/` with `python3 -m scripts.fig.as_pair_unrec_stacked_area`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/94>

Adopted from `as_unrec_stacked_area.py`.
"""
import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import download_if_missing

FILE = "as_pair_stats1.csv.gz"
TAGS = (
    "unrec_import_empty",
    "unrec_export_empty",
    "unrec_aut_num",
    "unrec_as_set_route",
    "unrec_some_as_set_route",
    "unrec_as_set",
    "unrec_as_routes",
    "unrec_route_set",
    "unrec_peering_set",
    "unrec_filter_set",
)


def plot():
    df = pd.read_csv(FILE)

    d = pd.DataFrame({"total": sum(df[tag] for tag in TAGS)})
    for tag in TAGS:
        d[f"%{tag}"] = df[tag] / d["total"] * 100.0
    d.dropna(inplace=True)
    d.sort_values(
        by=[f"%{tag}" for tag in TAGS],
        ascending=False,
        ignore_index=True,
        inplace=True,
    )

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        d.index,
        [d[f"%{tag}"] for tag in TAGS],
        labels=[f"%{tag}" for tag in TAGS],
    )
    ax.set_xlabel("AS Pair", fontsize=16)
    ax.set_ylabel(f"Percentage of Unrecorded Case", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="lower left", fontsize=14)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/files/13319676/as_pair_stats1.csv.gz",
        FILE,
    )
    fig, _, _ = plot()

    pdf_name = f"AS-pair-unrec-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
