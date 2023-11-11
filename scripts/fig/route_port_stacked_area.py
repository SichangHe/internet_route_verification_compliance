"""Run at `scripts/` with `python3 -m fig.route_port_stacked_area`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/88>
"""
import matplotlib.pyplot as plt
import pandas as pd
from fig import download_if_missing
from matplotlib.axes import Axes
from matplotlib.figure import Figure

FILE = "route_stats1.csv.gz"
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def plot():
    df = pd.read_csv(FILE, dtype="uint16")

    dfs: dict[str, pd.DataFrame] = {}
    figs: dict[str, Figure] = {}
    axs: dict[str, Axes] = {}
    for port in PORTS:
        d = pd.DataFrame({"total": sum(df[f"{port}_{tag}"] for tag in TAGS)})
        for tag in TAGS:
            d[f"%{tag}"] = df[f"{port}_{tag}"] / d["total"] * 100.0
        d.dropna(inplace=True)
        d.sort_values(
            by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
            ascending=[False, True, False, False, False],
            ignore_index=True,
            inplace=True,
        )
        dfs[port] = d
    d = pd.DataFrame(
        {"total": sum(df[f"{port}_{tag}"] for tag in TAGS for port in PORTS)}
    )
    for tag in TAGS:
        d[f"%{tag}"] = sum(df[f"{port}_{tag}"] for port in PORTS) / d["total"] * 100.0
    d.dropna(inplace=True)
    d.sort_values(
        by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
        ascending=[False, True, False, False, False],
        ignore_index=True,
        inplace=True,
    )
    dfs["exchange"] = d
    for key, d in dfs.items():
        fig, ax = plt.subplots(figsize=(16, 9))
        figs[key], axs[key] = fig, ax
        fig.tight_layout()
        ax.stackplot(
            d.index,
            [d[f"%{tag}"] for tag in TAGS],
            labels=[f"%{tag}" for tag in TAGS],
            rasterized=True,
        )
        ax.set_xlabel("Route", fontsize=16)
        ax.set_ylabel(f"Percentage of {key}", fontsize=16)
        ax.tick_params(axis="both", labelsize=14)
        ax.grid()
        ax.legend(loc="lower left", fontsize=14)

    # For checking.
    # figs["import"].show()

    return figs, axs, dfs


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-88/route_stats1.csv.gz",
        FILE,
    )
    figs, _, _ = plot()

    for key, fig in figs.items():
        pdf_name = f"route-{key}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(8, 6)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
