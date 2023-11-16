"""Run at `scripts/` with `python3 -m scripts.fig.as_rules_cdf`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/60#issuecomment-1751551274>
"""
import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import download_if_missing


def plot():
    df_raw = pd.read_csv("as_neighbors_vs_rules.csv")
    # Remove ASes not in IRR.
    df = df_raw.drop(df_raw[df_raw["import"] == -1].index)
    df["rules"] = df["import"] + df["export"]

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.ecdf(df["rules"], complementary=True, label="CCDF")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("Number of Import and Export Rules by Controlling AS", fontsize=16)
    ax.set_ylabel("Complementary Cumulative Fraction of ASes", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="best", fontsize=14)

    # For checking.
    # fig.show()

    return fig, ax


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/files/12908403/as_neighbors_vs_rules.csv",
        "as_neighbors_vs_rules.csv",
    )
    fig, _ = plot()

    pdf_name = "CDF-AS-rules.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
