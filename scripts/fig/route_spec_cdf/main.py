"""Run at `scripts/` with `python3 -m fig.route_port_cdf.main`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/72>
"""
import matplotlib.pyplot as plt
import pandas as pd
from fig import download_if_missing
from matplotlib.axes import Axes
from matplotlib.figure import Figure

FILE = "route_stats.csv.gz"
MEHS = (
    "spec_uphill",
    "spec_uphill_tier1",
    "spec_tier1_pair",
    "spec_import_peer_oifps",
    "spec_import_customer_oifps",
    "spec_export_customers",
    "spec_as_is_origin_but_no_route",
)


def plot():
    df = pd.read_csv(FILE, dtype="int16")
    meh_sums = sum(df[meh] for meh in MEHS)
    meh_percentages = pd.DataFrame(
        {f"%{meh}": df[meh] / meh_sums * 100.0 for meh in MEHS}
    )
    meh_percentages.dropna(inplace=True)

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    for meh in MEHS:
        ax.ecdf(df[meh], label=f"CDF of {meh}")
    ax.set_xlabel("Number of Special Cases by Controlling Routes", fontsize=16)
    ax.set_ylabel("Cumulative Fraction of Routes", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="best", fontsize=14)

    # For checking.
    # fig.show()

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    for meh in MEHS:
        ax.ecdf(meh_percentages[f"%{meh}"], label=f"CDF of %{meh}")
    ax.set_xlabel("Percentage of Special Cases by Controlling Routes", fontsize=16)
    ax.set_ylabel("Cumulative Fraction of Routes", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="best", fontsize=14)

    # For checking.
    # fig.show()

    return fig, ax


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-72/route_stats.csv.gz",
        FILE,
    )
    fig, _ = plot()

    pdf_name = f"route-special-cdf.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
