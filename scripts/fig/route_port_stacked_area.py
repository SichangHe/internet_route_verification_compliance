"""Run at `scripts/` with `python3 -m fig.route_port_stacked_area`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/72>
"""
import matplotlib.pyplot as plt
import pandas as pd
from fig import download_if_missing
from matplotlib.axes import Axes
from matplotlib.figure import Figure

FILE = "route_stats.csv.gz"
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def plot():
    df = pd.read_csv(FILE, dtype="int16")

    dfs: dict[str, pd.DataFrame] = {}
    figs: dict[str, Figure] = {}
    axs: dict[str, Axes] = {}
    for port in PORTS:
        d = pd.DataFrame({"total": sum(df[f"{port}_{tag}"] for tag in TAGS)})
        for tag in TAGS:
            d[f"%{tag}"] = df[f"{port}_{tag}"] / d["total"] * 100.0
        d.dropna(inplace=True)
        d = d.sort_values(
            by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
            ascending=[False, True, False, False, False],
            ignore_index=True,
        )
        dfs[port] = d
        figs[port], axs[port] = plt.subplots(figsize=(16, 9))
        fig, ax = figs[port], axs[port]
        fig.tight_layout()
        ax.stackplot(
            d.index,
            [d[f"%{tag}"] for tag in TAGS],
            labels=[f"%{tag}" for tag in TAGS],
            rasterized=True,
        )
        ax.set_xlabel("Route", fontsize=16)
        ax.set_ylabel(f"Percentage of {port}", fontsize=16)
        ax.tick_params(axis="both", labelsize=14)
        ax.grid()
        ax.legend(loc="lower left", fontsize=14)

    # For checking.
    # figs["import"].show()

    return figs, axs, dfs


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-72/route_stats.csv.gz",
        FILE,
    )
    figs, _, _ = plot()

    for port in PORTS:
        fig = figs[port]
        pdf_name = f"route-{port}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(8, 6)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
