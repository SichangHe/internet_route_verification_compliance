"""Run at `scripts/` with `python3 -m fig.as_pair_cdf`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/68>
"""
import matplotlib.pyplot as plt
import pandas as pd
from fig import download_if_missing
from matplotlib.axes import Axes
from matplotlib.figure import Figure

FILE = "as_pair_stats.csv"
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def plot():
    df = pd.read_csv(FILE)
    df["sum"] = sum(df[f"{port}_{tag}"] for port in PORTS for tag in TAGS)

    df_sum = df[["from", "to"]].copy()
    df_percentages = df[["from", "to"]].copy()
    for tag in TAGS:
        df_sum[tag] = sum(df[f"{port}_{tag}"] for port in PORTS)
        df_percentages[f"%{tag}"] = df_sum[tag] / df["sum"] * 100.0

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    for tag in TAGS:
        ax.ecdf(df_sum[tag], label=f"CDF of {tag}")
    ax.set_xlabel("Number of Import/Export by Controlling AS Pairs", fontsize=16)
    ax.set_ylabel("Cumulative Fraction of AS Pairs", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="best", fontsize=14)

    # For checking.
    # fig.show()

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    for tag in TAGS:
        ax.ecdf(df_percentages[f"%{tag}"], label=f"CDF of %{tag}")
    ax.set_xlabel("Percentages of Import/Export by Controlling AS Pairs", fontsize=16)
    ax.set_ylabel("Cumulative Fraction of AS Pairs", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="best", fontsize=14)

    # For checking.
    # fig.show()

    return fig, ax


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/files/12923141/as_pair_stats.csv",
        FILE,
    )
    fig, _ = plot()

    pdf_name = f"AS-pair-cdf.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
